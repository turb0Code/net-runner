pub use crate::packet_reciever;
pub use crate::game_interface;

use pnet::datalink;
use std::io;

pub fn program() -> Result<(), Box<dyn std::error::Error>>
{
    // display avaiable interfaces
    let interfaces = datalink::interfaces();
    println!("Avaiable interfaces: ");
    for iface in interfaces {
        println!("- Name: {}, Index: {}", iface.name, iface.index);
    }

    // picking interface to sniff
    println!("Provide interface name from list: ");
    let mut interface_name_raw = String::new();
    io::stdin().read_line(&mut interface_name_raw).expect("Failed to read line");
    let interface_name = interface_name_raw.trim();

    // getting interface
    let interface = packet_reciever::read_interfaces(interface_name);

    // opening packet reciever
    let mut rx = packet_reciever::open_reciever(&interface);

    // pick working mode
    let mut pick = String::new();
    println!("1. Packet sniffer \n2. Net hacking");
    println!("Choose mode (1 or 2): ");
    io::stdin().read_line(&mut pick).expect("Failed to read line");

    if pick.trim() == "1"
    {
        // recieving packets
        packet_reciever::recieve_packets(&mut *rx);
        Ok(())
    }
    else
    {
        println!("GAME TIME!");
        let _ = game_interface::main_interface();
        Ok(())
    }

}