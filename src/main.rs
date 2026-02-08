pub mod data_sources;

pub mod packet_reciever;
pub mod game_interface;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    core::matrix_mode::main_interface().await
}
