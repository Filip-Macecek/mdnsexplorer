use crate::mdns::capture;
use crate::mdnsexplorer_ui::mdns_message_table::MdnsMessageOverview;
use crate::mdnsexplorer_ui::mdnsexplorer_ui::{MdnsExplorerUi, ViewModel};
use std;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process;

pub struct MDNSExplorerApplication {
}

impl MDNSExplorerApplication {
    pub fn new() -> MDNSExplorerApplication {
        println!("Creating new MDNS explorer application");
        MDNSExplorerApplication {
        }
    }

    pub fn run(&mut self) {
        println!("Starting Mdns Explorer");
        let view_model = Arc::new(Mutex::new(ViewModel {
            mdns_message_overview_entries: vec![],
        }));
        thread::scope(|s| {
            s.spawn(|| {
                capture::start(|mdns_message| {
                    let model = MdnsMessageOverview::new(
                        mdns_message.header.query_identifier,
                        mdns_message.questions.iter().map(|q| q.name.clone()).collect(),
                        mdns_message.answers.iter().map(|a| a.name.clone()).collect()
                    );
                    match view_model.lock() {
                        Ok(mut m) => {
                            println!("Capture thread: {}", m.mdns_message_overview_entries.len());
                            m.mdns_message_overview_entries.push(model);
                        }
                        Err(_) => {}
                    }
                });
            });
            MdnsExplorerUi::run(&view_model);
            println!("Stopping Mdns Explorer");

            // Since the capture thread could potentially be blocked when awaiting packets,
            // this is the only way to properly end the program.
            process::exit(0);
        });
    }
}