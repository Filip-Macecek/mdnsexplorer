mod mdns;
mod mdnsexplorer_ui;
mod mdnsexplorer_app;

use pnet::packet::Packet;
use crate::mdnsexplorer_app::mdnsexplorer_app::MDNSExplorerApplication;

// TODO: Run as administrator.
fn main() {
    let mut application = MDNSExplorerApplication::new();
    application.run();
}