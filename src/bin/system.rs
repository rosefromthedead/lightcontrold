use std::time::Duration;

use color_eyre::Report;
use dbus::{arg::Variant, nonblock::Proxy};
use dbus_tokio::connection;
use lifx_more::{HSBK, Light, Message};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
    let (resource, conn) = connection::new_system_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("lost connection to D-Bus: {}", err);
    });

    let lights = Light::enumerate_v4(2000).await?;
    let light = &lights[0];

    light.send(Message::LightSetPower {
        level: 65535, // on
        duration: 1000,
    }).await?;

    Ok(())
}
