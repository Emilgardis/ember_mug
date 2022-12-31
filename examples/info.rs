#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .init();
    color_eyre::install()?;
    run().await
}

async fn run() -> Result<(), color_eyre::Report> {
    let mug = ember_mug::EmberMug::find_and_connect().await?;
    let mut chars: Vec<_> = mug.get_characteristics().collect();
    let known = ember_mug::KnownCharacteristic::all();
    for k in known {
        let Some(char) = chars.iter().position(|&c| &c.uuid == k.get()) else {
            println!("couldn't find {:?}", k);
            continue;
        };
        let char = chars.remove(char);
        println!("known: {k:?}\n{} cap: {:?}", char.uuid, char.properties);
    }
    for ch in chars {
        tracing::info!(?ch, "was left");
    }
    println!("battery: {:?}", mug.get_battery().await?);
    println!("current_temp: {}", mug.get_current_temperature().await?);
    println!("color: {:?}", mug.get_mug_color().await?);
    println!("name: {:?}", mug.get_name().await?);
    println!("state: {:?}", mug.get_liquid_state().await?);
    println!(
        "target_temperature: {}",
        mug.get_target_temperature().await?
    );
    println!("ota: {:?}", mug.get_ota().await?);
    println!("tdz: {:?}", mug.get_time_date_zone().await?);
    println!("meta: {:?}", mug.get_mug_meta().await?);
    println!("dsk: {:?}", mug.get_dsk().await?);
    println!("udsk: {:?}", mug.get_udsk().await?);

    Ok(())
}
