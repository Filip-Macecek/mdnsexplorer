use crate::mdns::capture;
use crate::mdnsexplorer_ui::mdns_message_table::MdnsMessageOverview;
use crate::mdnsexplorer_ui::mdnsexplorer_ui::{MdnsExplorerUi, ViewModel};
use std;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use pnet::datalink::{interfaces, NetworkInterface};
use crate::mdnsexplorer_ui::confirmation_dialogue_ui::ConfirmationDialogueUi;
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

        #[cfg(windows)]
        {
            use crate::mdnsexplorer_app::is_elevated::is_elevated;
            if !is_elevated()
            {
                ConfirmationDialogueUi::run(
                    "Missing Admin Privileges",
                    "Administrator privileges are missing. Please, make sure you are running the program as administrator."
                );
                process::exit(0);
            }
        }

        let interface = Self::run_interface_chooser();
        let view_model = Arc::new(Mutex::new(ViewModel {
            mdns_message_overview_entries: vec![],
            is_paused: false
        }));
        thread::scope(|s| {
            s.spawn(|| {
                capture::start(&interface, |mdns_message| {
                    let now = SystemTime::now();
                    match view_model.lock() {
                        Ok(mut m) => {
                            let duration = now.elapsed().unwrap();
                            // println!("Locking the view_model took {} ms.", duration.as_millis());

                            if !m.is_paused
                            {
                                let model = MdnsMessageOverview::new(
                                    mdns_message.received_datetime.time(),
                                    mdns_message.message.clone(),
                                    mdns_message.source_ip,
                                    mdns_message.destination_ip
                                );
                                m.mdns_message_overview_entries.push(model);
                            }
                        }
                        Err(_) => {
                            panic!("Could not lock Mdns message overview");
                        }
                    }
                });
            });
            MdnsExplorerUi::run(&view_model, &interface.description);
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