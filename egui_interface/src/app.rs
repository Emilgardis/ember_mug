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
    wanted_target_temp: f32,
}

#[derive(Debug)]
pub struct MugData {
    pub target_temp: f32,
    pub current_temp: f32,
    pub temp_unit: ember_mug::mug::TemperatureUnit,
    pub state: ember_mug::mug::LiquidState,
    pub battery: ember_mug::mug::Battery,
    pub liquid: ember_mug::mug::LiquidLevel,
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
        frame.set_window_size(ctx.used_size().max(egui::Vec2::new(200.0, 150.0)));
        'data: {
            if opt_mug.is_none() {
                let re = ctx.repaint_on_drop();
                if let Some(p) = resolver.try_take_with("device", async move {
                    let _re = re;

                    let mug = ember_mug::EmberMug::find_and_connect().await?;
                    mug.subscribe_push_events().await?;

                    let target_temp = mug.get_target_temperature().await?.to_degree();
                    let current_temp = mug.get_current_temperature().await?.to_degree();
                    let state = mug.get_liquid_state().await?;
                    let temp_unit = mug.get_temperature_unit().await?;
                    let liquid = mug.get_liquid_level().await?;
                    let battery = mug.get_battery().await?;
                    Ok::<_, color_eyre::Report>(Mug {
                        mug,
                        wanted_target_temp: target_temp,
                        data: MugData {
                            target_temp,
                            current_temp,
                            temp_unit,
                            state,
                            battery,
                            liquid,
                        },
                    })
                }) {
                    match p {
                        Ok(v) => {
                            tracing::info!("mug connected");
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
                            let liquid = mug.get_liquid_level().await?;
                            let temp_unit = mug.get_temperature_unit().await?;
                            let battery = mug.get_battery().await?;
                            sender.send(MugData {
                                target_temp,
                                current_temp,
                                state,
                                temp_unit,
                                battery,
                                liquid,
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
                            tracing::debug!("stream exited");
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
                        resolver.kill("listen_push_events");
                        std::mem::take(opt_mug);
                        break 'data;
                    }
                }
            }
        }

        if let Some(()) = opt_mug
            .as_ref()
            .and_then(|mug| {
                let mug = mug.mug.clone();
                let ctx = ctx.clone();

                resolver.try_take_with::<Result<_, ember_mug::btleplug::Error>, _>(
                    "check_alive",
                    async move {
                        let _repaint = ctx.clone();
                        mug.disconnected().await?;
                        ctx.request_repaint();
                        Ok(())
                    },
                )
            })
            .transpose()
            .unwrap()
        {
            tracing::info!("mug disconnected!");
            let a = std::mem::take(opt_mug);
            crate::runtime::spawn(async move { a });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(mug) = opt_mug {
                let data = &mut mug.data;
                ui.label(format!(
                    "Battery: {}, is {}charging",
                    data.battery.battery,
                    if !data.battery.charge { "not " } else { "" }
                ));
                ui.label(format!(
                    "Current temperature: {:.1}{}",
                    data.current_temp, data.temp_unit
                ));
                ui.label(format!(
                    "Target temperature: {:.1}{}",
                    data.target_temp, data.temp_unit
                ));
                ui.label(format!("State: {:?}", data.state));
                ui.label(format!("Liquid: {:?}", data.liquid));
                let needs_update: bool;
                if let Some(res) =
                    resolver.try_take::<Result<f32, color_eyre::Report>>("update_temp")
                {
                    tracing::debug!("temp updated");
                    mug.data.target_temp = res.unwrap();
                    tracing::debug!(?mug.wanted_target_temp, ?mug.data.target_temp);
                    needs_update = true;
                } else if (mug.wanted_target_temp - mug.data.target_temp).abs() > 0.05 {
                    tracing::trace!(?mug.wanted_target_temp, ?mug.data.target_temp);
                    needs_update = true;
                } else {
                    needs_update = false;
                }
                if ui
                    .add(
                        egui::widgets::Slider::new(&mut mug.wanted_target_temp, 50.0..=62.5)
                            .fixed_decimals(1),
                    )
                    .changed()
                    || needs_update
                {
                    let ctx_slider = ctx.clone();
                    let slider = resolver
                        .try_take_with("temp_update", async move {
                            let _repaint = ctx_slider.repaint_on_drop();
                            tokio::time::sleep(std::time::Duration::from_millis(400)).await;
                        })
                        .is_some();
                    if slider {
                        let target = mug.wanted_target_temp;
                        let ctx = ctx.clone();
                        let mug = mug.mug.clone();
                        if !resolver.exists(&"update_temp") {
                            resolver.add_with::<Result<f32, color_eyre::Report>, _>(
                                "update_temp",
                                async move {
                                    let _repaint = ctx.repaint_on_drop();
                                    mug.set_target_temperature(
                                        &ember_mug::mug::Temperature::from_degree(target),
                                    )
                                    .await?;
                                    mug.get_target_temperature()
                                        .await
                                        .map_err(Into::into)
                                        .map(|temp| temp.to_degree())
                                },
                            )
                        }
                    };
                }
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
