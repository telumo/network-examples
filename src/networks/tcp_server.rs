
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::{str, thread};
use failure;
use log::{error, debug};

pub fn serve(address: &str) -> Result<(), failure::Error> {

    // TCPのコネクションを待ち受けるソケットを作成
    // クライアントからのコネクション確立要求を待ち受けるだけ
    // リスニングソケット
    let listener = TcpListener::bind(address)?;

    loop {
        // 3 way handshake
        // コネクション確立済みのソケットをカーネル内部のキューに生成
        // 確立済みのソケットを返却
        // もし存在しなければ、生成するまでスレッドを停止させる
        // 接続済みソケット
        let (stream, _) = listener.accept()?;

        // コネクション毎にスレッドを立ち上げる
        // TODO: moveクロージャーとは？
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {

    debug!("Handling data from {}", stream.peer_addr()?);

    let mut buffer = [0u8; 1024];

    loop {
        // streamにデータがくるまで待機して、データが届いたら読み込んだバイト数を返す
        let nbytes = stream.read(&mut buffer)?;

        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }

        // デバック
        print!("{}", str::from_utf8(&buffer[..nbytes])?);

        // クライアントにオウム返しする
        stream.write_all(&buffer[..nbytes])?;
    }
}