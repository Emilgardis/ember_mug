# ember_mug | Rust crate for controlling and retrieving data from an Ember Ceramic Mug using Bluetooth Low Energy (BLE)

[![github]](https://github.com/emilgardis/ember_mug)&ensp;[![crates-io]](https://crates.io/crates/ember_mug)&ensp;[![docs-rs-big]](https://docs.rs/ember_mug/0.4.0/ember_mug/)

[github]: https://img.shields.io/badge/github-emilgardis/ember__mug-8da0cb?style=for-the-badge&labelColor=555555&logo=github

You can see current unpublished docs here: [![local-docs]](https://emilgardis.github.io/ember_mug/ember_mug)

[local-docs]: https://img.shields.io/github/actions/workflow/status/emilgardis/ember_mug/gh-pages.yml?branch=main
[crates-io]: https://img.shields.io/crates/v/ember_mug.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-rs-big]: https://img.shields.io/badge/docs.rs-ember__mug-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

Provides a Rust interface for interacting with Ember Mug devices through the [btleplug](https://crates.io/crates/btleplug) crate. Used to create applications that can control and monitor Ember Mug devices; retrieving the current temperature or battery level, setting the target temperature or mug color and accessing device metadata.

## Example

```rust ,no_run
use ember_mug::{mug::Temperature, EmberMug};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mug = EmberMug::find_and_connect().await?;
    let name = mug.get_name().await?;
    println!("Connected to an Ember Mug with the name '{}'", name);

    let battery = mug.get_battery().await?;
    println!("Battery level: {}%", battery.battery);

    let current_temp = mug.get_current_temperature().await?;
    let target_temp = mug.get_target_temperature().await?;
    let unit = mug.get_temperature_unit().await?;
    println!("Current temperature: {}{}", current_temp, unit);
    println!("Target temperature: {}{}", target_temp, unit);

    mug.set_target_temperature(&Temperature::from_degree(60.0))
        .await?;

    let target_temp = mug.get_target_temperature().await?;
    println!("Changed target temperature to {}{}", target_temp, unit);

    Ok(())
}
```

Results in

```text
Connected to an Ember Mug with the name 'EMBER'
Battery level: 100%
Current temperature: 20.5°C
Target temperature: 54°C
Changed target temperature to 60°C
```

## Kudos

Thanks to [orlopau/ember-mug](https://github.com/orlopau/ember-mug) and [sopelj/python-ember-mug](https://github.com/sopelj/python-ember-mug) for providing hints towards the various BLE characteristics

<h5> License </h5>

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
