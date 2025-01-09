use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;

fn main(){
    env::set_var("RUST_LOG","debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    if args.len() != 5{
        error!("Please specify [tcp|udp] [server|client] [addr:port]");
        std::process::exit(1);
    }
    let protocol: &str = &args[2];
    let role: &str = &args[3];
    let _address: &str = &args[4];
    match protocol{
        "tcp" => match role{
            "server" => {
                //TODO:CALL TCP SERVER
                tcp_server::serve(_address).unwrap_or_else(|e|error!("{}",e));
                //println!("TCPサーバ");
            }
            "client" => {
                //TODO:CALL TCP CLIENT
                tcp_client::connect(_address).unwrap_or_else(|e|error!("{}",e));
                //println!("TCPクライアント");
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role{
            "server" => {
                //TODO:CALL UDP SERVER
                println!("UDPサーバ");
            }
            "client" => {
                //TODO:CALL UDP CLIENT
                println!("UDPクライアント");
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

/**
 * 第二引数が不正なときにエラーを出す関数
 * missing_role()
 */
///第二引数が不正なときにエラーを出す関数
fn missing_role(){
    error!("Please specify server or alient on the 2nd argument.");
    std::process::exit(1);
}