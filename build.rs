fn main() {
    // Specify the path to the directory containing `packet.lib`
    println!("cargo:rustc-link-search=native=libs");
}