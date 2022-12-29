/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct EmberMugApp {
    settings: Settings,
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
    pub temp_unit: ember_mug::TemperatureUnit,
    pub state: ember_mug::LiquidState,
    pub battery: ember_mug::Battery,
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
            resolver,
        } = self;
        resolver.poll();
        ctx.request_repaint();

        if mug.is_none() {
            let ctx_c = ctx.clone();
            if let Some(p) = resolver.try_take_with("device", async move {
                let _a = defer(move || ctx_c.request_repaint());

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
                        mug.replace(v);
                    }
                    Err(e) => {
                        tracing::warn!(error=?e, "got error");
                        ctx.request_repaint();
                    }
                }
            }
        } else if let Some(mug) = mug {
            let mug_mug = mug.mug.clone();
            if let Some(update) = resolver
                .try_stream_with::<_, _, color_eyre::Report>("event", move |sender| {
                    let mug = mug_mug.clone();
                    let ctx = ctx.clone();
                    async move {
                        let mut interval =
                            tokio::time::interval(std::time::Duration::from_secs(10));
                        loop {
                            interval.tick().await;
                            let target_temp = mug.get_target_temperature().await?.to_degree();
                            let current_temp = mug.get_current_temperature().await?.to_degree();
                            let state = mug.get_liquid_state().await?;
                            let temp_unit = mug.get_temperature_unit().await?;
                            let battery = mug.get_battery().await?;
                            ctx.request_repaint();
                            sender.send(MugData {
                                target_temp,
                                current_temp,
                                state,
                                temp_unit,
                                battery,
                            })?;
                        }
                    }
                })
                .unwrap()
            {
                tracing::debug!(?update, "update");
                mug.data = update;
            }
            let mug_mug = mug.mug.clone();
            if let Some(event) = resolver
                .try_stream_with::<_, _, color_eyre::Report>("listen_push_events", |sender| {
                    let mug = mug_mug.clone();
                    let ctx = ctx.clone();
                    tracing::debug!("event stream initialize");

                    async move {
                        use futures::StreamExt;
                        let mut events = mug.listen_push_events().await?.boxed();
                        tracing::debug!("listening to events");
                        while let Some(event) = events.next().await {
                            let event = match event {
                                Ok(e) => crate::events::PushEvent::new(&mug, e).await,
                                Err(_) => todo!(),
                            };
                            tracing::debug!(?event, "got event");

                            sender.send(event)?;
                            ctx.request_repaint();
                        }
                        Ok(())
                    }
                })
                .unwrap()
            {
                tracing::debug!(?event, "got event");
                match event {
                    Ok(event) => event.update(&mut mug.data),
                    Err(e) => tracing::error!(err = ?e),
                }
            }
        }

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // if let Some(e) = &promises.device_fail {
            //     ui.label(e.to_string());
            // }
            if let Some(mug) = mug {
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
