use core::str;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread};

//指定のソケットアドレスで待ち受ける
pub fn serve(_address: &str) -> Result<(),failure::Error>{
    let listener = TcpListener::bind(_address)?;
    loop{
        let (stream,_) = listener.accept()?;
        //スレッドを立ち上げて接続対応
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error|error!("{:?}",error));
        });
    }
}

//クライアントからの入力を待ち受け，受信したら同じものを返す
fn handler(mut stream: TcpStream) -> Result<(),failure::Error>{
    debug!("Handling data from {}",stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop{
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0{
            debug!("Connection closed.");
            return Ok(());
        }
        
        let ja_string: String = String::from_utf8(buffer[..nbytes].to_vec())?;
        let cp_string: String = chipanese(ja_string.clone());
        println!("日本語:{}", ja_string);
        println!("偽中国語:{}", cp_string);
        stream.write_all(&cp_string.into_bytes())?;
    }
}

fn chipanese(japanese_string: String) -> String {
    let chipanese_string = japanese_string
        .chars()
        .filter(|&c| matches!(c,'一' ..= '鿿'|'。'|'、'))
        .collect::<String>() + "\n";

    //assert_eq!(str,"カタカナ"); //なんだこれ

    return chipanese_string;
}