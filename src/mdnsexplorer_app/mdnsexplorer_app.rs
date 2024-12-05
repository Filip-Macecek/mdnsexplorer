use crate::mdns::capture;
use crate::mdnsexplorer_ui::mdns_message_table::MdnsMessageOverview;
use crate::mdnsexplorer_ui::mdnsexplorer_ui::{MdnsExplorerUi, ViewModel};
use pnet::packet::Packet;
use std;
use std::sync::{Arc, Mutex};
use std::thread;

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
        let mut view_model = Arc::new(Mutex::new(ViewModel {
            mdns_message_overview_entries: vec![],
        }));
        thread::scope(|s| {
            // TODO: Don't know how to kill it yet.
            s.spawn(|| {
                capture::start(|mdns_message| {
                    let model = MdnsMessageOverview::new(
                        mdns_message.query_identifier,
                        vec!(),
                        vec!()
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
        });
    }
}