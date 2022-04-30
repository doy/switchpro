use tokio_stream::StreamExt as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let controller = std::env::args().nth(1).unwrap().parse().unwrap();

    let (_, session) = bluez_async::BluetoothSession::new().await?;
    session.start_discovery().await?;

    let mut stream = session.event_stream().await?;
    while let Some(event) = stream.next().await {
        if let bluez_async::BluetoothEvent::Device { id, .. } = &event {
            let info = session.get_device_info(id).await?;
            if info.mac_address == controller {
                session.connect(id).await?;
                break;
            }
        }
    }

    Ok(())
}
