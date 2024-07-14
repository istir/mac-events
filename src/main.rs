pub mod config;
pub mod mirror;
pub mod monitor;
pub mod usb;

#[tokio::main]
async fn main() {
    let handler = tokio::spawn(async {
        usb::UsbHandler::listen();
    });
    let _ = handler.await;
}
