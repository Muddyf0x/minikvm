use std::net::{IpAddr, Ipv4Addr};
use sysfs_gpio::Pin;
use minikvm::Server;
use std::io;

fn main() -> ! {
    let server = Server::new(Pin::new(8), Pin::new(7), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 178)));
    loop {
        println!("1. Start
2. reboot 
3. shutdwon
4. get up");
        let mut com = String::new();
        io::stdin()
            .read_line(&mut com)
            .expect("Failed to read line");
            let com: u32 = match com.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        
        match com {
            1 => server.start(),
            2 => server.restart(),
            3 => server.shutdown(),
            _ => (),
        };
    }
}

