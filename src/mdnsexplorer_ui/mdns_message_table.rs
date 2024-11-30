use eframe::egui;

pub struct MdnsMessageTable {
    striped: bool,
    resizable: bool,
    clickable: bool,
    num_rows: usize,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
    selection: std::collections::HashSet<usize>,
    checked: bool,
    reversed: bool,
}

impl Default for MdnsMessageTable {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            clickable: true,
            num_rows: 10_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
            selection: Default::default(),
            checked: false,
            reversed: false
        }
    }
}

impl MdnsMessageTable {
    pub fn render(&mut self, ui: &mut egui::Ui, reset: bool) {
        use egui_extras::{Column, TableBuilder};

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        // if let Some(row_index) = self.scroll_to_row.take() {
        //     table = table.scroll_to_row(row_index, None);
        // }

        if reset {
            table.reset();
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Row");
                        },
                        |ui| {
                            // self.reversed ^=
                            //     ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("Clipped text");
                });
                header.col(|ui| {
                    ui.strong("Expanding content");
                });
                header.col(|ui| {
                    ui.strong("Interaction");
                });
                header.col(|ui| {
                    ui.strong("Content");
                });
            })
            .body(|mut body| {
                let row_height = |i: usize| if thick_row(i) { 30.0 } else { 18.0 };
                body.heterogeneous_rows((0..self.num_rows).map(row_height), |mut row| {
                    let row_index = if self.reversed {
                        self.num_rows - 1 - row.index()
                    } else {
                        row.index()
                    };

                    row.set_selected(self.selection.contains(&row_index));

                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });
                    row.col(|ui| {
                        ui.label(long_text(row_index));
                    });
                    row.col(|ui| {
                        expanding_content(ui);
                    });
                    row.col(|ui| {
                        // ui.checkbox(&mut self.checked, "Click me");
                    });
                    row.col(|ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        if thick_row(row_index) {
                            ui.heading("Extra thick row");
                        } else {
                            ui.label("Normal row");
                        }
                    });

                    self.toggle_row_selection(row_index, &row.response());
                });
            });
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }
}

fn expanding_content(ui: &mut egui::Ui) {
    ui.add(egui::Separator::default().horizontal());
}

fn long_text(row_index: usize) -> String {
    format!("Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!")
}

fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}