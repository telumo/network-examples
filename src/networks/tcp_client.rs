use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

pub fn connect(address: &str) -> Result<(), failure::Error>{

    // コネクションを作成。この段階で自動的に3 way handshakeが行われる。
    let mut stream = TcpStream::connect(address)?;

    loop{
        // 文字列の箱を用意
        let mut input = String::new();
        // ユーザーの入力をinputに代入
        io::stdin().read_line(&mut input)?;
        // ストリームにバイトとして流す（バイトで送る）
        stream.write_all(input.as_bytes())?;

        // 比較的小さいデータのやり取り時に使用される。Readと違いsystem callを呼び出さない。
        let mut reader = BufReader::new(& stream);
        // 箱を用意
        let mut buffer = Vec::new();
        // bufferの中に読み取った値を代入
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}", str::from_utf8(&buffer)?);
    }
}