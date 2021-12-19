use std::time::Duration;

use color_eyre::Report;
use dbus::{arg::Variant, nonblock::Proxy};
use dbus_tokio::connection;
use lifx_more::{Light, HSBK};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
    let (resource, conn) = connection::new_session_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("lost connection to D-Bus: {}", err);
    });

    let lights = Light::enumerate_v4(2000).await?;
    let light = &lights[0];

    let mut interval = tokio::time::interval(Duration::from_secs(30));

    let proxy = Proxy::new("org.kde.KWin", "/ColorCorrect", Duration::from_secs(1), conn);
    let calls = async move {
        loop {
            interval.tick().await;
            let temp: (Variant<u32>,) = proxy.method_call("org.freedesktop.DBus.Properties", "Get", ("org.kde.kwin.ColorCorrect", "currentTemperature")).await?;

            light.send(lifx_more::Message::LightSetColor {
                reserved: 0,
                color: HSBK { hue: 0, saturation: 0, brightness: 40000, kelvin: temp.0.0 as u16 },
                duration: 10000,
            }).await?;
        }
        #[allow(unreachable_code)]
        Result::<(), Report>::Ok(())
    };

    calls.await?;

    Ok(())
}
