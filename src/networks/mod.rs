use std::env;

use log::{info, error};
use types::*;

mod cli;
mod tcp_client;
pub mod tcp_server;
pub mod types;
mod udp_client;
pub mod udp_server;

pub fn run() {
    // 環境変数を設定
    env::set_var("RUST_LOG", "debug");

    env_logger::init();
    let arguments = cli::get_arguments();

    match arguments.protocol {
        Protocol::Tcp => match arguments.role {
            Role::Client => {
                info!("Switch to TCP Client");
                tcp_client::connect(&arguments.address).unwrap_or_else(|e| error!("{}", e));
            }
            Role::Server => {
                info!("Switch to TCP Server");
                tcp_server::serve(&arguments.address).unwrap_or_else(|e| error!("{}", e));
            }
        },
        Protocol::Udp => match arguments.role {
            Role::Client => {
                info!("Switch to UDP Client");
            }
            Role::Server => {
                info!("Switch to TCP Server");
                udp_server::serve(&arguments.address).unwrap_or_else(|e| error!("{}", e));
            }
        },
    }
}
