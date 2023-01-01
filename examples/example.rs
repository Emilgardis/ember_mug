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
