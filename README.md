# Ember Mug | Rust crate for interacting with an Ember Ceramic Mug

[![github]](https://github.com/emilgardis/ember_mug)

[github]: https://img.shields.io/badge/github-emilgardis/ember__mug-8da0cb?style=for-the-badge&labelColor=555555&logo=github

You can see current unpublished docs here: [![local-docs]](https://emilgardis.github.io/ember_mug/ember_mug)

[local-docs]: https://img.shields.io/github/actions/workflow/status/emilgardis/ember_mug/gh-pages.yml?branch=main

This crate provides a Rust interface for interacting with Ember Mug devices. It provides access to the various characteristics of an Ember Mug, such as temperature, battery level, and more. It also provides a convenience struct for representing an Ember Mug device and interacting with it through the [btleplug](https://crates.io/crates/btleplug) crate. This crate is useful for developers looking to create applications that can control and monitor Ember Mug devices, such as by retrieving the current temperature or battery level, setting the target temperature or mug color, or accessing device metadata.

## Example

```rust ,no_run
use ember_mug::{EmberMug, Temperature};

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
