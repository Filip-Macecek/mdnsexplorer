use crate::mdnsexplorer_ui::mdns_message_table::{MdnsMessageOverview, MdnsMessageTable};
use eframe::egui;
use egui::TextStyle;
use egui_extras::{Size, StripBuilder};
use std::sync::{Arc, Mutex};

pub struct ViewModel {
    pub mdns_message_overview_entries: Vec<MdnsMessageOverview>
}

pub struct MdnsExplorerUi<'l> {
    name: String, // Test
    age: u32,
    view_model: &'l Arc<Mutex<ViewModel>>
}

impl MdnsExplorerUi<'_> {
    pub fn run(view_model: &Arc<Mutex<ViewModel>>) {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };
        let _ = eframe::run_native(
            "My egui App",
            options,
            Box::new(|_| {
                Ok(Box::<MdnsExplorerUi>::new(MdnsExplorerUi {
                    age: 42,
                    name: "Filip".parse().unwrap(),
                    view_model: view_model
                }))
            }),
        );
    }

    pub fn get(&self) -> Vec<MdnsMessageOverview>
    {
        match self.view_model.lock() {
            Ok(m) => {
                println!("UI Thread Get Called: {}", m.mdns_message_overview_entries.len());
                m.mdns_message_overview_entries.clone()
            }
            Err(_) => { panic!("Nope.")}
        }
    }
}

impl eframe::App for MdnsExplorerUi<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("Mdns Explorer Update");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.separator();
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            let reset = false;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
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
