use rusb::{Context, Device, HotplugBuilder, UsbContext};

use crate::{config::Config, monitor::Monitor};
pub struct UsbHandler;
impl UsbHandler {
    pub fn listen() {
        let monitor = Monitor::new();
        let context = Context::new().unwrap();
        let device_ids = Config::new().data.device_ids.hex;
        let mut hotplug = HotplugBuilder::new();

        for device_id in device_ids {
            if device_id > 0 {
                hotplug.product_id(device_id);
            }
        }

        let mut _reg = hotplug
            .register(
                context.clone(),
                Box::new(HotPlugHandler { monitor: monitor }),
            )
            .unwrap();
        loop {
            context.handle_events(None).unwrap();
        }
    }
}

struct HotPlugHandler {
    monitor: Monitor,
}

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        println!("device arrived {:?}", device);
        self.monitor.turn_on();
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("device left {:?}", device);
        self.monitor.turn_off();
    }
}
