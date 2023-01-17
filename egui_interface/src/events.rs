#[derive(Debug)]
pub enum PushEvent {
    RefreshBatteryLevel(ember_mug::mug::Battery),
    Charging(bool),
    NotCharging(bool),
    RefreshTargetTemperature(ember_mug::mug::Temperature),
    RefreshDrinkTemperature(ember_mug::mug::Temperature),
    AuthInfoNotFound(),
    RefreshLiquidLevel(ember_mug::mug::LiquidLevel),
    RefreshLiquidState(ember_mug::mug::LiquidState),
    BatteryVoltageState(ember_mug::mug::Battery),
}
impl PushEvent {
    pub(crate) async fn new(
        mug: &ember_mug::mug::EmberMug,
        event: ember_mug::mug::PushEvent,
    ) -> Result<Self, ember_mug::ReadError> {
        match event {
            ember_mug::mug::PushEvent::RefreshBatteryLevel => {
                let v = mug.get_battery().await?;
                Ok(Self::RefreshBatteryLevel(v))
            }
            ember_mug::mug::PushEvent::Charging => Ok(Self::Charging(true)),
            ember_mug::mug::PushEvent::NotCharging => Ok(Self::NotCharging(false)),
            ember_mug::mug::PushEvent::RefreshTargetTemperature => {
                let v = mug.get_target_temperature().await?;
                Ok(Self::RefreshTargetTemperature(v))
            }
            ember_mug::mug::PushEvent::RefreshDrinkTemperature => {
                let v = mug.get_current_temperature().await?;
                Ok(Self::RefreshDrinkTemperature(v))
            }
            ember_mug::mug::PushEvent::AuthInfoNotFound => Ok(Self::AuthInfoNotFound()),
            ember_mug::mug::PushEvent::RefreshLiquidLevel => {
                let v = mug.get_liquid_level().await?;
                Ok(Self::RefreshLiquidLevel(v))
            }
            ember_mug::mug::PushEvent::RefreshLiquidState => {
                let v = mug.get_liquid_state().await?;
                Ok(Self::RefreshLiquidState(v))
            }
            ember_mug::mug::PushEvent::BatteryVoltageState => {
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
            PushEvent::RefreshLiquidLevel(liquid) => {
                data.liquid = liquid;
            }
            PushEvent::RefreshLiquidState(state) => data.state = state,
            PushEvent::BatteryVoltageState(battery) => data.battery = battery,
        }
    }
}
