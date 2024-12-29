use std::sync::{Arc, Mutex};
use eframe::egui;
use egui::{Vec2, ViewportCommand};
use pnet::datalink::NetworkInterface;

pub struct InterfaceChooserUi {
    interfaces: Vec<NetworkInterface>,
    picked_interface: Arc<Mutex<Option<NetworkInterface>>>
}

impl InterfaceChooserUi {
    pub fn run(interfaces: Vec<NetworkInterface>, picked_interface: Arc<Mutex<Option<NetworkInterface>>>) {
        let builder = egui::ViewportBuilder::default()
            .with_maximize_button(false)
            .with_inner_size(Vec2::new(400.0, 250.0))
            .with_close_button(false)
            .with_always_on_top();
        let options = eframe::NativeOptions {
            viewport: builder,
            ..Default::default()
        };
        let _ = eframe::run_native(
            "Choose interface",
            options,
            Box::new(|_| {
                Ok(Box::<InterfaceChooserUi>::new(InterfaceChooserUi { interfaces, picked_interface }))
            }),
        );
    }
}

impl eframe::App for InterfaceChooserUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MDNS Explorer");
            ui.separator();
            ui.vertical_centered(|ui| {
                for interface in self.interfaces.iter() {
                    if ui.button(&interface.description).clicked() {
                        match self.picked_interface.try_lock() {
                            Ok(mut picked_interface) => { *picked_interface = Some(interface.clone()); },
                            Err(e) => { panic!("Unable to lock picked interface."); }
                        }
                        ctx.send_viewport_cmd(ViewportCommand::Close)
                    };
                }
            })
        });
    }
}
