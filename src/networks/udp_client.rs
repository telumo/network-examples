

use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {

    // おそらく、同一マシーンで異なるサーバを立てているから違うポート番号にする必要がある。
    let ip_address = extract_only_address(address);
    let socket_addr = format!("{}:0", ip_address);
    let socket = UdpSocket::bind(socket_addr)?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        // アドレスは送信先のアドレスとポート番号
        socket.send_to(input.as_bytes(), address)?;

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to recieve");
        print!(
            "{}", str::from_utf8(&buffer).expect("faild to cenvert to String")
        );
    }
}

fn extract_only_address(address: &str) -> &str {
    address.split(":").collect::<Vec<&str>>()[0]
}

#[cfg(test)]
mod test {
    use super::extract_only_address;

    #[test]
    fn test_extract_only_address_1() {
        let ret = extract_only_address("127.0.0.1:3333");
        assert_eq!(ret, "127.0.0.1");
    }
}