use crate::mdns::capture;
use crate::mdnsexplorer_ui::mdns_message_table::MdnsMessageOverview;
use crate::mdnsexplorer_ui::mdnsexplorer_ui::{MdnsExplorerUi, ViewModel};
use std;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use pnet::datalink::{interfaces, NetworkInterface};
use crate::mdnsexplorer_ui::interface_chooser_ui::InterfaceChooserUi;

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

        let interface = Self::run_interface_chooser();
        let view_model = Arc::new(Mutex::new(ViewModel {
            mdns_message_overview_entries: vec![],
        }));
        thread::scope(|s| {

            s.spawn(|| {
                capture::start(&interface, |mdns_message| {
                    let model = MdnsMessageOverview::new(
                        mdns_message.received_datetime.time(),
                        mdns_message.message.clone(),
                    );
                    let now = SystemTime::now();
                    match view_model.lock() {
                        Ok(mut m) => {
                            let duration = now.elapsed().unwrap();
                            println!("Locking the view_model took {} ms.", duration.as_millis());
                            m.mdns_message_overview_entries.push(model);
                        }
                        Err(_) => {
                            panic!("Could not lock Mdns message overview");
                        }
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

    fn run_interface_chooser() -> NetworkInterface
    {
        let interfaces = interfaces();
        let picked_interface = Arc::new(Mutex::new(None));
        InterfaceChooserUi::run(
            interfaces.clone(),
            picked_interface.clone()
        );
        let interface = match picked_interface.lock() {
            Ok(i) => i.clone().expect("No interface was picked."),
            Err(_) => {
                panic!("Could not lock picked interface.");
            }
        };
        return interface;
    }
}