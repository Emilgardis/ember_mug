use super::*;
impl EmberMug {
    /// Events sent by the mug for the application to register to.
    ///
    /// Call [`subscribe_push_events`](Self::unsubscribe_push_events) first, and prefer to use [`listen_push_events`](Self::listen_push_events) instead
    pub async fn get_push_event(&self) -> Result<PushEvents, ReadError> {
        PushEvents::read(&mut Cursor::new(self.read(&PUSH_EVENTS).await?)).map_err(Into::into)
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
    ) -> Result<impl futures::stream::Stream<Item = Result<PushEvents, ReadError>> + '_, ReadError>
    {
        use futures::StreamExt;
        let stream = self
            .peripheral
            .notifications()
            .await?
            .filter_map(move |v| async move {
                if v.uuid == PUSH_EVENTS {
                    match self.read(&PUSH_EVENTS).await {
                        Ok(b) => Some(PushEvents::read(&mut Cursor::new(b)).map_err(Into::into)),
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
#[br(repr = u8)]
#[br(little)]
pub enum PushEvents {
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
