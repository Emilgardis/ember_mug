use super::*;
impl EmberMug {
    /// Events sent by the mug for the application to register to.
    ///
    /// Call [`subscribe_push_events`](Self::unsubscribe_push_events) first, and prefer to use [`listen_push_events`](Self::listen_push_events) instead
    pub async fn get_push_event(&self) -> Result<PushEvent, ReadError> {
        PushEvent::read(&mut Cursor::new(self.read(&PUSH_EVENTS).await?)).map_err(Into::into)
    }
    /// Subscribe to events sent by the mug
    pub async fn subscribe_push_events(&self) -> Result<(), ReadError> {
        self.peripheral
            .subscribe(
                self.get_characteristic(&PUSH_EVENTS)
                    .ok_or(ReadError::NoSuchCharacteristic)?,
            )
            .await
            .map_err(Into::into)
    }

    /// Get a stream of events sent by the mug
    pub async fn listen_push_events(
        &self,
    ) -> Result<
        impl futures::stream::Stream<Item = Result<PushEvent, ReadError>> + Send + '_,
        ReadError,
    > {
        use futures::StreamExt;
        self.subscribe_push_events().await?;
        let stream = self
            .peripheral
            .notifications()
            .await?
            .filter_map(move |v| async move {
                if v.uuid == PUSH_EVENTS {
                    match self.read(&PUSH_EVENTS).await {
                        Ok(b) => Some(PushEvent::read(&mut Cursor::new(b)).map_err(Into::into)),
                        Err(e) => Some(Err(e)),
                    }
                } else {
                    tracing::debug!(%v.uuid, ?v.value, "received unknown event");
                    None
                }
            });
        Ok(stream)
    }

    /// Unsubscribe to events sent by the mug
    pub async fn unsubscribe_push_events(&self) -> Result<(), ReadError> {
        self.peripheral
            .unsubscribe(
                self.get_characteristic(&PUSH_EVENTS)
                    .ok_or(ReadError::NoSuchCharacteristic)?,
            )
            .await
            .map_err(Into::into)
    }
}

#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "snake_case"))]
#[br(repr = u8)]
#[br(little)]
pub enum PushEvent {
    /// Refresh battery level
    RefreshBatteryLevel = 0x01,
    /// Charging
    Charging = 0x02,
    /// Not Charging
    NotCharging = 0x03,
    /// Refresh target temperature
    RefreshTargetTemperature = 0x04,
    /// Refresh drink temperature
    RefreshDrinkTemperature = 0x05,
    /// Auth info missing
    AuthInfoNotFound = 0x06,
    /// Refresh liquid level
    RefreshLiquidLevel = 0x07,
    /// Refresh liquid state
    RefreshLiquidState = 0x08,
    /// Battery voltage state changed
    BatteryVoltageState = 0x09,
}

impl PushEvent {
    /// Returns `true` if the push event is [`RefreshBatteryLevel`].
    ///
    /// [`RefreshBatteryLevel`]: PushEvent::RefreshBatteryLevel
    #[must_use]
    pub fn is_refresh_battery_level(&self) -> bool {
        matches!(self, Self::RefreshBatteryLevel)
    }

    /// Returns `true` if the push event is [`Charging`].
    ///
    /// [`Charging`]: PushEvent::Charging
    #[must_use]
    pub fn is_charging(&self) -> bool {
        matches!(self, Self::Charging)
    }

    /// Returns `true` if the push event is [`NotCharging`].
    ///
    /// [`NotCharging`]: PushEvent::NotCharging
    #[must_use]
    pub fn is_not_charging(&self) -> bool {
        matches!(self, Self::NotCharging)
    }

    /// Returns `true` if the push event is [`RefreshTargetTemperature`].
    ///
    /// [`RefreshTargetTemperature`]: PushEvent::RefreshTargetTemperature
    #[must_use]
    pub fn is_refresh_target_temperature(&self) -> bool {
        matches!(self, Self::RefreshTargetTemperature)
    }

    /// Returns `true` if the push event is [`RefreshDrinkTemperature`].
    ///
    /// [`RefreshDrinkTemperature`]: PushEvent::RefreshDrinkTemperature
    #[must_use]
    pub fn is_refresh_drink_temperature(&self) -> bool {
        matches!(self, Self::RefreshDrinkTemperature)
    }

    /// Returns `true` if the push event is [`AuthInfoNotFound`].
    ///
    /// [`AuthInfoNotFound`]: PushEvent::AuthInfoNotFound
    #[must_use]
    pub fn is_auth_info_not_found(&self) -> bool {
        matches!(self, Self::AuthInfoNotFound)
    }

    /// Returns `true` if the push event is [`RefreshLiquidLevel`].
    ///
    /// [`RefreshLiquidLevel`]: PushEvent::RefreshLiquidLevel
    #[must_use]
    pub fn is_refresh_liquid_level(&self) -> bool {
        matches!(self, Self::RefreshLiquidLevel)
    }

    /// Returns `true` if the push event is [`RefreshLiquidState`].
    ///
    /// [`RefreshLiquidState`]: PushEvent::RefreshLiquidState
    #[must_use]
    pub fn is_refresh_liquid_state(&self) -> bool {
        matches!(self, Self::RefreshLiquidState)
    }

    /// Returns `true` if the push event is [`BatteryVoltageState`].
    ///
    /// [`BatteryVoltageState`]: PushEvent::BatteryVoltageState
    #[must_use]
    pub fn is_battery_voltage_state(&self) -> bool {
        matches!(self, Self::BatteryVoltageState)
    }

    pub fn event_name(&self) -> &'static str {
        match self {
            PushEvent::RefreshBatteryLevel => "refresh_battery_level::event",
            PushEvent::Charging => "charging::event",
            PushEvent::NotCharging => "not_charging::event",
            PushEvent::RefreshTargetTemperature => "refresh_target_temperature::event",
            PushEvent::RefreshDrinkTemperature => "refresh_drink_temperature::event",
            PushEvent::AuthInfoNotFound => "auth_info_not_found::event",
            PushEvent::RefreshLiquidLevel => "refresh_liquid_level::event",
            PushEvent::RefreshLiquidState => "refresh_liquid_state::event",
            PushEvent::BatteryVoltageState => "battery_voltage_state::event",
        }
    }
}
