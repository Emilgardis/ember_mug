# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/emilgardis/ember_mug/compare/v0.4.0...Unreleased)

## [v0.4.0] - 2023-11-28

[Commits](https://github.com/emilgardis/ember_mug/compare/v0.3.0...v0.4.0)

- Properly read push events
- Make errors for reading and writing more descriptive when it's the underlying bluetooth stack that is failing
  - Replaced `WriteError::BtleError` with `WriteError::WriteOperation`, added `ReadError::ReadOperation` and `ReadError::SubscribeOperation`

## [v0.3.0] - 2023-01-23

[Commits](https://github.com/emilgardis/ember_mug/compare/v0.2.0...v0.3.0)

- Read and write functions now take a `&ember_mug::KnownCharacteristic` instead of `&uuid::Uuid`

## [v0.2.0] - 2023-01-21

[Commits](https://github.com/emilgardis/ember_mug/compare/v0.1.1...v0.2.0)

- Change display format of `TemperatureUnit` to include `°` and drop `°` from `Temperature` display format
- `ember_mug::btle::get_mugs()` now returns the adapter that the peripheral is on
  and `ember_mug::btle::get_mugs_on_adapters()` returns the index to the adapter.
- `ember_mug::btle::search_adapter_for_ember()` uses event driven discovery for quicker discovery, now returns a stream.
- `ember_mug::EmberMug::connect_mug()` now requires the adapter for where the peripheral exists
- Added `ember_mug::EmberMug::is_connected()` to check the status of the mug and `ember_mug::EmberMug::disconnected()` to catch disconnection.
- Added `ember_mug::EMBER_MUG_PUBLIC_SERVICES` for known services broadcasted by mugs.

## [v0.1.1] - 2023-01-01

[Commits](https://github.com/emilgardis/ember_mug/compare/v0.1.0...v0.1.1)

- When all `EmberMug` and its clones have been dropped, disconnect the device.

## [v0.1.0] - 2023-01-01

- Initial release.

## [End of Changelog]

Changelog starts on v0.1.0
