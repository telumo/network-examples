use std::env;

use log::{info};
use types::*;

mod cli;
mod tcp_client;
mod tcp_server;
pub mod types;
mod udp_client;
mod udp_server;

pub fn run() {
    // 環境変数を設定
    env::set_var("RUST_LOG", "debug");

    env_logger::init();
    let arguments = cli::get_arguments();

    match arguments.protocol {
        Protocol::Tcp => match arguments.role {
            Role::Client => {
                info!("Switch to TCP Client")
            }
            Role::Server => {
                info!("Switch to TCP Server")
            }
        },
        Protocol::Udp => match arguments.role {
            Role::Client => {
                info!("Switch to UDP Client")
            }
            Role::Server => {
                info!("Switch to TCP Server")
            }
        },
    }
}
