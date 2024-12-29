use eframe::egui;
use egui::{Vec2, ViewportCommand};

pub struct ConfirmationDialogueUi {
    message: String
}

impl ConfirmationDialogueUi {
    pub fn run(title: &str, message: &str) {
        let builder = egui::ViewportBuilder::default()
            .with_maximize_button(false)
            .with_inner_size(Vec2::new(250.0, 120.0))
            .with_close_button(true)
            .with_always_on_top();
        let options = eframe::NativeOptions {
            viewport: builder,
            ..Default::default()
        };
        let _ = eframe::run_native(
            title,
            options,
            Box::new(|_| {
                Ok(Box::<ConfirmationDialogueUi>::new(ConfirmationDialogueUi {
                    message: message.to_string()
                }))
            }),
        );
    }
}

impl eframe::App for ConfirmationDialogueUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MDNS Explorer");
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.label(&self.message);

                if ui.button("OK").clicked()
                {
                    ctx.send_viewport_cmd(ViewportCommand::Close)
                }
            })
        });
    }
}
