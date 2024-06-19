
fn main() {
    println!("cargo:rustc-link-search=native=C:\\Windows\\System32\\Npcap"); 
    println!("cargo:rustc-link-lib=wpcap"); 
}
