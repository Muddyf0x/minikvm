use minikvm::{auto, manuel, Server};
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use sysfs_gpio::Pin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let server = Server::new(
        Pin::new(8),
        Pin::new(7),
        IpAddr::V4(Ipv4Addr::new(192, 168, 0, 178)),
    );
    let mut mode = parse_config(args);
    let mut rev = 0;
    loop {
        if rev == 1 {
        } else {
            rev += 1;
        }
        if mode == 0 {
            mode = manuel(&server);
        } else if mode == 1 {
            auto(&server);
        } else if mode < 0 {
            break;
        }
    }
}

fn parse_config(args: Vec<String>) -> i8 {
    match args.len() {
        1 => return 0,
        2 => (),
        _ => return -1,
    };
    match args[1].as_str() {
        "--auto" => 1,
        "-auto" => 1,
        _ => 0,
    }
}
