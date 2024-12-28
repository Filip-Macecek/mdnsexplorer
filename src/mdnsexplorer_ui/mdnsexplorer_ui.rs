use crate::mdnsexplorer_ui::mdns_message_table::{MdnsMessageOverview, MdnsMessageTable};
use eframe::egui;
use egui::{TextStyle, Vec2};
use egui_extras::{Size, StripBuilder};
use std::sync::{Arc, Mutex};

pub struct ViewModel {
    pub mdns_message_overview_entries: Vec<MdnsMessageOverview>
}

pub struct MdnsExplorerUi<'l> {
    view_model: &'l Arc<Mutex<ViewModel>>
}

impl MdnsExplorerUi<'_> {
    pub fn run(view_model: &Arc<Mutex<ViewModel>>) {
        let builder = egui::ViewportBuilder::default()
            .with_maximize_button(true)
            .with_inner_size(Vec2::new(1200.0, 800.0));
        let options = eframe::NativeOptions {
            viewport: builder,
            ..Default::default()
        };
        let _ = eframe::run_native(
            "My egui App",
            options,
            Box::new(|_| {
                Ok(Box::<MdnsExplorerUi>::new(MdnsExplorerUi {
                    view_model: view_model
                }))
            }),
        );
    }

    pub fn get(&self) -> Vec<MdnsMessageOverview>
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
}

impl eframe::App for MdnsExplorerUi<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MDNS Explorer");
            ui.separator();
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            let reset = false;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table.
                .size(Size::exact(body_text_size)) // for the source code link.
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            MdnsMessageTable::new(self.get()).render(ui, reset);
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
