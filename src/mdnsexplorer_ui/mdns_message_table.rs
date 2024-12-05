use std::cell::Ref;
use eframe::egui;

pub struct MdnsMessageOverview {
    id: u16,
    questions: Vec<String>,
    answers: Vec<String>
}

impl MdnsMessageOverview {
    pub fn new(id: u16, questions: Vec<String>, answers: Vec<String>) -> Self {
        Self { id, questions, answers }
    }
}

impl Clone for MdnsMessageOverview {
    fn clone(&self) -> Self {
        MdnsMessageOverview::new(self.id, self.questions.clone(), self.answers.clone())
    }
}

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
    overviews: Vec<MdnsMessageOverview>
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
            reversed: false,
            overviews: Default::default()
        }
    }
}

impl MdnsMessageTable {
    pub fn new(a: Vec<MdnsMessageOverview>) -> Self {
        Self {
            striped: true,
            resizable: true,
            clickable: true,
            num_rows: 10_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
            selection: Default::default(),
            checked: false,
            reversed: false,
            overviews: a
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, reset: bool) {
        use egui_extras::{Column, TableBuilder};
        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        if reset {
            table.reset();
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Questions");
                });
                header.col(|ui| {
                    ui.strong("Answers");
                });
            })
            .body(|mut body| {
                let row_height = |i: usize| if thick_row(i) { 30.0 } else { 18.0 };
                let _ = body.rows(30f32, self.overviews.len(), | mut row| {
                    let row_index = row.index();

                    // row.set_selected(self.selection.contains(&row_index));
                    let overview = &self.overviews[row_index];
                    print!("{}, {}", overview.questions.join(", "), overview.answers.join(", "));
                    row.col(|ui| {
                        ui.label(overview.questions.join(", "));
                    });
                    row.col(|ui| {
                        ui.label(overview.answers.join(", "));
                    });
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