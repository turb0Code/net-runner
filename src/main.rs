extern crate pnet;

pub mod sniffing_mode;
pub mod sending_mode;

pub mod packet_reciever;
pub mod game_interface;

use std::path::Path;

fn is_npcap_installed() -> bool {
    #[cfg(windows)]
    {
        // Sprawdzamy standardowe lokalizacje System32 lub folderu Npcap
        let system_path = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
        let dll_path = format!("{}\\System32\\wpcap.dll", system_path);

        // Można też sprawdzić obecność sterownika w rejestrze lub plikach
        Path::new(&dll_path).exists() || Path::new("C:\\Program Files\\Npcap\\NPFInstall.exe").exists()
    }
    #[cfg(not(windows))]
    true // Na Linux/macOS pcap jest zazwyczaj częścią systemu
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if is_npcap_installed()
    // if false  // DEBUG MODE
    {
        sniffing_mode::program()?;
    } else {
        println!("Npcap is not installed. Please install Npcap to use this application.");
    }
    Ok(())
}
