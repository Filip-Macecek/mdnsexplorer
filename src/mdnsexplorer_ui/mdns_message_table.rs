use std::cmp;
use std::cmp::max;
use crate::mdns::mdns_message::MDNSMessage;
use crate::mdns::types::{MDNSAnswer, MDNSQuestion};
use eframe::egui;
use time::Time;

#[derive(Clone)]
pub struct MdnsMessageOverview {
    utc_time: Time,
    message: MDNSMessage
}

impl MdnsMessageOverview {
    pub fn new(utc_time: Time, message: MDNSMessage) -> Self {
        Self { utc_time, message }
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
            resizable: false,
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
            .column(Column::initial(120 as f32).resizable(true).auto_size_this_frame(false))
            .column(Column::initial(500 as f32).resizable(true).auto_size_this_frame(false))
            .column(Column::initial(500 as f32).resizable(true).auto_size_this_frame(false))
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .stick_to_bottom(true);

        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        if reset {
            table.reset();
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("UTC Time");
                });
                header.col(|ui| {
                    ui.strong("Questions");
                });
                header.col(|ui| {
                    ui.strong("Answers");
                });
            })
            .body(|body| {
                let row_height = 30f32;
                let _ = body.heterogeneous_rows(self.overviews.iter().map(|o| Self::get_row_height(&o)), | mut row| {
                    let row_index = row.index();
                    let overview = &self.overviews[row_index];
                    row.col(|ui| {
                        ui.label(overview.utc_time.to_string());
                    });
                    let questions = overview.message.questions.iter().map(|q| Self::format_question(q)).collect::<Vec<_>>();
                    row.col(|ui| {
                        ui.label(questions.join("\n"));
                    });
                    let answers = overview.message.answers.iter().map(|a| Self::format_answer(a)).collect::<Vec<_>>();
                    row.col(|ui| {
                        ui.label(answers.join("\n"));
                    });
                });
            });
    }

    fn get_row_height(overview: &MdnsMessageOverview) -> f32
    {
        let default_height = 20f32;
        let len = max(overview.message.answers.len(), overview.message.questions.len());
        return max(len, 1) as f32 * default_height;
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

    fn format_question(question: &MDNSQuestion) -> String
    {
        return format!("{}: {}", question.question_type.to_string(), question.name);
    }

    fn format_answer(answer: &MDNSAnswer) -> String
    {
        return format!("{}: {} => {}", answer.answer_type.to_string(), answer.name, answer.rdata.to_string());
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