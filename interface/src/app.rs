use std::sync::Arc;

use poll_promise::Promise;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct EmberMugApp {
    settings: Settings,
    #[serde(skip)]
    mug: Option<Mug>,
    #[serde(skip)]
    promises: Promises,
}

pub struct Mug {
    mug: ember_mug::EmberMug,
    data: Arc<tokio::sync::RwLock<MugData>>,
}
pub struct MugData {
    target_temp: f32,
    current_temp: f32,
    temp_unit: ember_mug::TemperatureUnit,
    state: ember_mug::LiquidState,
    battery: ember_mug::Battery,
}
#[derive(Default)]
struct Promises {
    device: Option<Promise<Result<Mug, color_eyre::Report>>>,
    streams: Option<(
        flume::Receiver<ember_mug::PushEvent>,
        tokio::task::AbortHandle,
    )>,
    join_set: tokio::task::JoinSet<Result<(), color_eyre::Report>>,
    device_fail: Option<color_eyre::Report>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            label: "Ember mug app".to_owned(),
            value: 42.0,
        }
    }
}

impl EmberMugApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        };

        Default::default()
    }
}

impl eframe::App for EmberMugApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            settings,
            mug,
            promises,
        } = self;

        if mug.is_none() {
            if let Some(p) = promises.device.take() {
                match p.try_take() {
                    Ok(Ok(v)) => {
                        let (sender, receiver) = flume::unbounded();
                        let mug_c = v.mug.clone();
                        let abort = {
                            let ctx = ctx.clone();
                            promises.join_set.spawn(async move {
                                use futures::{FutureExt, StreamExt};
                                let sender = sender.clone();
                                let mug = mug_c;
                                let mut events = mug.listen_push_events().await?.boxed();
                                while let Some(event) = events.next().await {
                                    sender.send(event?)?;
                                    ctx.request_repaint();
                                }
                                Ok::<_, color_eyre::Report>(())
                            })
                        };
                        let data = v.data.clone();
                        let mug_c = v.mug.clone();
                        {
                            let ctx = ctx.clone();
                            let abort = promises.join_set.spawn(async move {
                                let mut interval =
                                    tokio::time::interval(std::time::Duration::from_secs(10));
                                loop {
                                    interval.tick().await;
                                    let target_temp =
                                        mug_c.get_target_temperature().await?.to_degree();
                                    let current_temp =
                                        mug_c.get_current_temperature().await?.to_degree();
                                    let state = mug_c.get_liquid_state().await?;
                                    let temp_unit = mug_c.get_temperature_unit().await?;
                                    let battery = mug_c.get_battery().await?;
                                    let mut data = data.write().await;
                                    data.target_temp = target_temp;
                                    data.current_temp = current_temp;
                                    data.state = state;
                                    data.temp_unit = temp_unit;
                                    data.battery = battery;
                                    ctx.request_repaint();
                                }
                                Ok(())
                            });
                        }
                        promises.streams = Some((receiver, abort));
                        mug.replace(v);
                        promises.device_fail = None;
                    }
                    Ok(Err(e)) => {
                        promises.device_fail = Some(e);
                        ctx.request_repaint();
                    }
                    Err(p) => {
                        promises.device = Some(p);
                    }
                }
            } else {
                let ctx = ctx.clone();
                promises.device = Some(poll_promise::Promise::spawn_async(async move {
                    let _a = defer(move || ctx.request_repaint());

                    let mug = ember_mug::EmberMug::find_and_connect().await?;

                    let target_temp = mug.get_target_temperature().await?.to_degree();
                    let current_temp = mug.get_current_temperature().await?.to_degree();
                    let state = mug.get_liquid_state().await?;
                    let temp_unit = mug.get_temperature_unit().await?;
                    let battery = mug.get_battery().await?;
                    Ok(Mug {
                        mug,
                        data: Arc::new(
                            MugData {
                                target_temp,
                                current_temp,
                                temp_unit,
                                state,
                                battery,
                            }
                            .into(),
                        ),
                    })
                }));
            }
        } else if let Some((recv, _)) = &promises.streams {
            let mug = mug.as_ref().unwrap();
            for event in recv.drain() {
                let data = mug.data.clone();
                let mug = mug.mug.clone();
                dbg!(&event);
                match event {
                    ember_mug::PushEvent::RefreshBatteryLevel => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let v = mug.get_battery().await?;
                            let mut data = data.write().await;
                            data.battery = v;
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::Charging => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let mut data = data.write().await;
                            data.battery.charge = true;
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::NotCharging => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let mut data = data.write().await;
                            data.battery.charge = false;
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::RefreshTargetTemperature => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let v = mug.get_target_temperature().await?;
                            let mut data = data.write().await;
                            data.target_temp = v.to_degree();
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::RefreshDrinkTemperature => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let v = mug.get_current_temperature().await?;
                            let mut data = data.write().await;
                            data.current_temp = v.to_degree();
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::AuthInfoNotFound => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            dbg!("oops");
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::RefreshLiquidLevel => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let _v = mug.get_liquid_level().await?;
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::RefreshLiquidState => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let v = mug.get_liquid_state().await?;
                            let mut data = data.write().await;
                            data.state = v;
                            Ok(())
                        });
                    }
                    ember_mug::PushEvent::BatteryVoltageState => {
                        let ctx = ctx.clone();
                        promises.join_set.spawn(async move {
                            let _a = defer(move || ctx.request_repaint());

                            let v = mug.get_battery().await?;
                            let mut data = data.write().await;
                            data.battery = v;
                            Ok(())
                        });
                    }
                }
            }
        }

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            if let Some(e) = &promises.device_fail {
                ui.label(e.to_string());
            }
            if let Some(mug) = mug {
                let data = tokio::task::block_in_place(|| mug.data.blocking_read());
                ui.label(format!(
                    "Battery: {}, is {}charging",
                    data.battery.battery,
                    if !data.battery.charge { "not " } else { "" }
                ));
                ui.label(format!(
                    "Current temperature: {}{}",
                    data.current_temp, data.temp_unit
                ));
                ui.label(format!(
                    "Target temperature: {}{}",
                    data.target_temp, data.temp_unit
                ));
                ui.label(format!("State: {:?}", data.state));
            }
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

fn defer<F: FnOnce()>(f: F) -> impl Drop {
    struct D<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for D<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f()
            }
        }
    }
    D(Some(f))
}
