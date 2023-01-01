use crate::repaint_on_drop::Repaint;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct EmberMugApp {
    #[serde(skip)]
    mug: Option<Mug>,
    #[serde(skip)]
    resolver: crate::runtime::Resolver<&'static str>,
}

pub struct Mug {
    mug: ember_mug::EmberMug,
    data: MugData,
}

#[derive(Debug)]
pub struct MugData {
    pub target_temp: f32,
    pub current_temp: f32,
    pub temp_unit: ember_mug::mug::TemperatureUnit,
    pub state: ember_mug::mug::LiquidState,
    pub battery: ember_mug::mug::Battery,
}

impl EmberMugApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        };

        Default::default()
    }
}

impl eframe::App for EmberMugApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            mug: opt_mug,
            resolver,
        } = self;
        resolver.poll();
        frame.set_window_size(egui::Vec2::new(180.0, 90.0));
        'data: {
            if opt_mug.is_none() {
                let re = ctx.repaint_on_drop();
                if let Some(p) = resolver.try_take_with("device", async move {
                    let _re = re;

                    let mug = ember_mug::EmberMug::find_and_connect().await?;

                    let target_temp = mug.get_target_temperature().await?.to_degree();
                    let current_temp = mug.get_current_temperature().await?.to_degree();
                    let state = mug.get_liquid_state().await?;
                    let temp_unit = mug.get_temperature_unit().await?;
                    let battery = mug.get_battery().await?;
                    Ok::<_, color_eyre::Report>(Mug {
                        mug,
                        data: MugData {
                            target_temp,
                            current_temp,
                            temp_unit,
                            state,
                            battery,
                        },
                    })
                }) {
                    match p {
                        Ok(v) => {
                            opt_mug.replace(v);
                        }
                        Err(e) => {
                            tracing::warn!(error=?e, "got error");
                        }
                    }
                }
            } else if let Some(mug) = opt_mug {
                let mug_mug = mug.mug.clone();
                match resolver.try_stream_with::<_, _, color_eyre::Report>("event", move |sender| {
                    let ctx = ctx.clone();
                    let mug = mug_mug.clone();
                    async move {
                        let mut interval =
                            tokio::time::interval(std::time::Duration::from_secs(10));
                        loop {
                            let _re = ctx.repaint_on_drop();
                            interval.tick().await;
                            let target_temp = mug.get_target_temperature().await?.to_degree();
                            let current_temp = mug.get_current_temperature().await?.to_degree();
                            let state = mug.get_liquid_state().await?;
                            let temp_unit = mug.get_temperature_unit().await?;
                            let battery = mug.get_battery().await?;
                            sender.send(MugData {
                                target_temp,
                                current_temp,
                                state,
                                temp_unit,
                                battery,
                            })?;
                        }
                    }
                }) {
                    Ok(Some(update)) => {
                        tracing::debug!(?update, "update");
                        mug.data = update;
                    }
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!(error = ?e, "got error");
                        std::mem::take(opt_mug);
                        break 'data;
                    }
                }
                let mug_mug = mug.mug.clone();
                match resolver.try_stream_with::<_, _, color_eyre::Report>(
                    "listen_push_events",
                    |sender| {
                        let mug = mug_mug.clone();
                        let ctx = ctx.clone();
                        tracing::debug!("event stream initialize");

                        async move {
                            let _re = ctx.repaint_on_drop();
                            use futures::StreamExt;
                            let mut events = mug.listen_push_events().await?.boxed();
                            tracing::debug!("listening to events");
                            while let Some(event) = events.next().await {
                                let _re = ctx.repaint_on_drop();
                                let event = match event {
                                    Ok(e) => crate::events::PushEvent::new(&mug, e).await,
                                    Err(_) => todo!(),
                                };
                                tracing::debug!(?event, "got event");
                                sender.send(event)?;
                            }
                            Ok(())
                        }
                    },
                ) {
                    Ok(Some(event)) => {
                        tracing::debug!(?event, "got event");
                        match event {
                            Ok(event) => event.update(&mut mug.data),
                            Err(e) => tracing::error!(err = ?e),
                        }
                    }
                    Ok(None) => (),
                    Err(e) => {
                        tracing::error!(error = ?e, "got error");
                        std::mem::take(opt_mug);
                        break 'data;
                    }
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(mug) = opt_mug {
                let data = &mug.data;
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
            } else {
                ui.label("connecting to mug");
            }
        });
    }

    fn persist_native_window(&self) -> bool {
        false
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}
