use tokio::net::UdpSocket;
use trust_dns_client::{
    client::{AsyncClient, ClientHandle},
    rr::{DNSClass, Name, RecordType},
    udp::UdpClientStream,
};
use trust_dns_client::serialize::binary::BinEncodable;

pub async fn send_dns_query(server: &str, domain: &str, record_type: RecordType,) -> anyhow::Result<Vec<u8>> // np. "8.8.8.8:53" np. "google.com"
{
    let stream = UdpClientStream::<UdpSocket>::new(server.parse()?);
    let (mut client, bg) = AsyncClient::connect(stream).await?;
    tokio::spawn(bg);

    let name = Name::from_ascii(domain)?;
    let response = client
        .query(name, DNSClass::IN, record_type)
        .await?;

    // SUROWE BAJTY odpowiedzi
    Ok(response.to_bytes()?)
}

pub async fn send_https_request(url: &str) -> anyhow::Result<Vec<u8>>
{
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "net-runner/1.0")
        .send()
        .await?;

    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

pub async fn send_raw_udp(server: &str, payload: &[u8]) -> anyhow::Result<Vec<u8>> // np. "1.1.1.1:53"
{
    let socket = UdpSocket::bind("1.1.1.1:53").await?;
    socket.send_to(payload, server).await?;

    let mut buf = vec![0u8; 512];
    let (len, _) = socket.recv_from(&mut buf).await?;
    buf.truncate(len);

    Ok(buf)
}
