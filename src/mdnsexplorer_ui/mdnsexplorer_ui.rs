use crate::mdnsexplorer_ui::mdns_message_table::{MdnsMessageOverview, MdnsMessageTable};
use eframe::egui;
use egui::{TextStyle, Vec2};
use egui_extras::{Size, StripBuilder};
use std::sync::{Arc, Mutex};

pub struct ViewModel {
    pub mdns_message_overview_entries: Vec<MdnsMessageOverview>,
    pub is_paused: bool
}

pub struct MdnsExplorerUi<'l> {
    view_model: &'l Arc<Mutex<ViewModel>>,
    interface_name: String
}

impl MdnsExplorerUi<'_> {
    pub fn run(view_model: &Arc<Mutex<ViewModel>>, interface_name: &str) {
        let builder = egui::ViewportBuilder::default()
            .with_maximize_button(true)
            .with_inner_size(Vec2::new(1300.0, 800.0));
        let options = eframe::NativeOptions {
            viewport: builder,
            ..Default::default()
        };
        let _ = eframe::run_native(
            "MDNS Explorer",
            options,
            Box::new(|_| {
                Ok(Box::<MdnsExplorerUi>::new(MdnsExplorerUi {
                    view_model: view_model,
                    interface_name: interface_name.to_string()
                }))
            }),
        );
    }

    fn get_overviews(&self) -> Vec<MdnsMessageOverview>
    {
        match self.view_model.lock() {
            Ok(m) => {
                m.mdns_message_overview_entries.clone()
            }
            Err(_) => {
                panic!("Nope.")
            }
        }
    }

    fn is_paused(&self) -> bool
    {
        match self.view_model.lock() {
            Ok(m) => {
                m.is_paused
            }
            Err(_) => {
                panic!("Nope.")
            }
        }
    }

    fn pause(&mut self, is_paused: bool) {
        match self.view_model.lock() {
            Ok(mut m) => {
                m.is_paused = is_paused
            }
            Err(_) => {
                panic!("Nope.")
            }
        }
    }
}

impl eframe::App for MdnsExplorerUi<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MDNS Explorer");
            ui.label(format!("Capturing on interface: {}", self.interface_name));

            let is_paused = self.is_paused();
            let pause_button_label = if is_paused { "Unpause" } else { "Pause" };
            if ui.button(pause_button_label).clicked() {
                self.pause(!is_paused);
            }
            ui.separator();
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            let reset = false;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table.
                .size(Size::exact(body_text_size)) // for the source code link.
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            MdnsMessageTable::new(self.get_overviews()).render(ui, reset);
                        });
                    });
                    strip.cell(|ui| {
                        ui.vertical_centered(|_| {
                        });
                    });
                });
        });
    }
}
