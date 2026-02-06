use rand::Rng;
use anyhow::Result;
use trust_dns_client::rr::RecordType;
pub mod packet_creator;

use crate::data_sources::sending::packet_creator::{
    send_dns_query,
    send_https_request,
    send_raw_udp,
};

#[derive(Debug, Clone, Copy)]
pub enum SendingVariant {
    DnsA,
    DnsTxt,
    Https,
    RawUdp,
}

pub fn random_variant() -> SendingVariant {
    match rand::thread_rng().gen_range(0..4) {
        0 => SendingVariant::DnsA,
        1 => SendingVariant::DnsTxt,
        2 => SendingVariant::Https,
        _ => SendingVariant::RawUdp,
    }
}


pub async fn fetch_random_packet() -> Result<Vec<u8>> {
    let variant = random_variant();

    let bytes = match variant {
        SendingVariant::DnsA => {
            send_dns_query(
                "8.8.8.8:53",
                "google.com",
                RecordType::A,
            )
            .await?
        }

        SendingVariant::DnsTxt => {
            send_dns_query(
                "1.1.1.1:53",
                "cloudflare.com",
                RecordType::TXT,
            )
            .await?
        }

        SendingVariant::Https => {
            send_https_request(
                "https://httpbin.org/uuid"
            )
            .await?
        }

        SendingVariant::RawUdp => {
            let payload = b"\x13\x37\xDE\xAD\xBE\xEF";
            send_raw_udp(
                "1.1.1.1:53",
                payload
            )
            .await?
        }
    };

    Ok(bytes)
}

