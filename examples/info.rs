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
    println!("battery: {:?}", mug.get_battery().await?);
    println!("current_temp: {}", mug.get_current_temperature().await?);
    println!("color: {:?}", mug.get_mug_color().await?);
    println!("name: {:?}", mug.get_name().await?);
    println!(
        "target_temperature: {}",
        mug.get_target_temperature().await?
    );
    println!("ota: {:?}", mug.get_ota().await?);
    println!("ota: {:?}", mug.get_mug_meta().await?);
    println!("dsk: {:?}", mug.get_dsk().await?);
    println!("udsk: {:?}", mug.get_udsk().await?);

    Ok(())
}
