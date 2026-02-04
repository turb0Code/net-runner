// using pnet to read network packages
use pnet::datalink::{self, NetworkInterface, DataLinkReceiver};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;

pub fn read_interfaces(interface_name: &str) -> NetworkInterface
{
    let interfaces = datalink::interfaces();

    interfaces.into_iter()
        .find(|iface: &NetworkInterface| iface.name == interface_name)
        .expect("Could't find such interace")
}

pub fn open_reciever(interface: &NetworkInterface) -> Box<dyn DataLinkReceiver> 
{
    match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => rx, // _tx ignorujemy, jeśli tylko czytamy
        Ok(_) => panic!("Nieobsługiwany typ kanału"),
        Err(e) => panic!("Błąd tworzenia kanału: {}", e),
    }
}

pub fn recieve_packets(rx: &mut dyn DataLinkReceiver)
{
    loop {
        match rx.next() {
            Ok(packet) => {
                // 3. Mapujemy surowe bajty na strukturę EthernetPacket
                if let Some(eth_packet) = EthernetPacket::new(packet) {
                    handle_packet(&eth_packet);
                }
            }
            Err(e) => panic!("Error while recieving packet: {}", e),
        }
    }
}

fn handle_packet(eth_packet: &EthernetPacket) {
    match eth_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            // Przechodzimy warstwę głębiej do IP
            if let Some(ipv4) = Ipv4Packet::new(eth_packet.payload()) {
                println!(
                    "IPv4: {} -> {} | Protocol: {:?} | Length: {}",
                    ipv4.get_source(),
                    ipv4.get_destination(),
                    ipv4.get_next_level_protocol(),
                    ipv4.get_total_length()
                );
            }
        }
        EtherTypes::Arp => println!("ARP packet"),
        _ => {} // Ignorujemy inne typy
    }
}

pub fn recieve_packet(rx: &mut dyn DataLinkReceiver)
{
    
}
