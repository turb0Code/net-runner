pub mod data_sources;

pub mod packet_reciever;
pub mod game_interface;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    loop {
        let result = core::matrix_mode::main_interface().await;

        match result {
            Ok(core::matrix_mode::AppExit::Quit) => {
                break;
            },
            Ok(core::matrix_mode::AppExit::Reload) => {
                continue;
            }
            Err(e) => {
                eprintln!("An error occurred: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
