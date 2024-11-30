use eframe::egui;
use egui::{TextStyle, Widget};
use egui_extras::{Size, StripBuilder};
use crate::mdnsexplorer_ui::mdns_message_table::MdnsMessageTable;

pub struct MdnsExplorerUi {
    name: String,
    age: u32,
}

impl Default for MdnsExplorerUi {
    fn default() -> Self {
        Self {
            name: "Ahoj".to_owned(),
            age: 0,
        }
    }
}

impl eframe::App for MdnsExplorerUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            let mut reset = false;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            let mut a = MdnsMessageTable::default();
                            a.render(ui, reset);
                        });
                    });
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                        });
                    });
                });
        });
    }
}
