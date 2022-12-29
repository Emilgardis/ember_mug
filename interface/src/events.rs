#[derive(Debug)]
pub enum PushEvent {
    RefreshBatteryLevel(ember_mug::Battery),
    Charging(bool),
    NotCharging(bool),
    RefreshTargetTemperature(ember_mug::Temperature),
    RefreshDrinkTemperature(ember_mug::Temperature),
    AuthInfoNotFound(),
    RefreshLiquidLevel(ember_mug::LiquidLevel),
    RefreshLiquidState(ember_mug::LiquidState),
    BatteryVoltageState(ember_mug::Battery),
}
impl PushEvent {
    pub(crate) async fn new(
        mug: &ember_mug::EmberMug,
        event: ember_mug::PushEvent,
    ) -> Result<Self, ember_mug::ReadError> {
        match event {
            ember_mug::PushEvent::RefreshBatteryLevel => {
                let v = mug.get_battery().await?;
                Ok(Self::RefreshBatteryLevel(v))
            }
            ember_mug::PushEvent::Charging => Ok(Self::Charging(true)),
            ember_mug::PushEvent::NotCharging => Ok(Self::NotCharging(false)),
            ember_mug::PushEvent::RefreshTargetTemperature => {
                let v = mug.get_target_temperature().await?;
                Ok(Self::RefreshTargetTemperature(v))
            }
            ember_mug::PushEvent::RefreshDrinkTemperature => {
                let v = mug.get_current_temperature().await?;
                Ok(Self::RefreshDrinkTemperature(v))
            }
            ember_mug::PushEvent::AuthInfoNotFound => Ok(Self::AuthInfoNotFound()),
            ember_mug::PushEvent::RefreshLiquidLevel => {
                let v = mug.get_liquid_level().await?;
                Ok(Self::RefreshLiquidLevel(v))
            }
            ember_mug::PushEvent::RefreshLiquidState => {
                let v = mug.get_liquid_state().await?;
                Ok(Self::RefreshLiquidState(v))
            }
            ember_mug::PushEvent::BatteryVoltageState => {
                let v = mug.get_battery().await?;
                Ok(Self::BatteryVoltageState(v))
            }
        }
    }

    pub(crate) fn update(self, data: &mut crate::app::MugData) {
        tracing::debug!(?self, "updating stuff");
        match self {
            PushEvent::RefreshBatteryLevel(battery) => data.battery = battery,
            PushEvent::Charging(b) => data.battery.charge = b,
            PushEvent::NotCharging(b) => data.battery.charge = b,
            PushEvent::RefreshTargetTemperature(target) => data.target_temp = target.to_degree(),
            PushEvent::RefreshDrinkTemperature(current) => data.current_temp = current.to_degree(),
            PushEvent::AuthInfoNotFound() => (),
            PushEvent::RefreshLiquidLevel(_) => {
                tracing::debug!("got refresh of level, doing nothing")
            }
            PushEvent::RefreshLiquidState(state) => data.state = state,
            PushEvent::BatteryVoltageState(battery) => data.battery = battery,
        }
    }
}
