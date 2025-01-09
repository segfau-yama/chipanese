use std::io::{self,BufRead,BufReader,Write};
use std::net::TcpStream;
use std::str;

///指定のIPアドレス、ポート番号にTCP接続。
pub fn connect(_address: &str) -> Result<(), failure::Error>{
    let mut stream = TcpStream::connect(_address)?;
    loop{
        //入力データをソケットから送信。
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;
        //ソケットから受信したデータを表示
        let mut reader= BufReader::new(&stream);
        let mut buffer= Vec::new();
        reader.read_until(b'\n',&mut buffer)?;// \nまで読み込み
        print!("output:{}",str::from_utf8(&buffer)?);
    }
}