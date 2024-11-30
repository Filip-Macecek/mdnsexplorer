use crate::mdns::capture;
use crate::mdnsexplorer_ui::mdnsexplorer_ui::MdnsExplorerUi;
use eframe::egui;
use pnet::packet::Packet;
use std;
use std::thread;

pub struct MDNSExplorerApplication { }

impl MDNSExplorerApplication {
    pub fn new() -> MDNSExplorerApplication {
        MDNSExplorerApplication { }
    }

    pub fn start(&self) {
        thread::spawn(move || {
            capture::start()
        });

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };
        eframe::run_native(
            "My egui App",
            options,
            Box::new(|cc| {
                Ok(Box::<MdnsExplorerUi>::default())
            }),
        );
    }
}