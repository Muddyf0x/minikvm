use ping::ping;
use std::io;
use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

pub struct Server {
    reset: Pin,
    power: Pin,
    ip: IpAddr,
    pub status: bool,
}

impl Server {
    pub fn new(reset: Pin, power: Pin, ip: IpAddr) -> Server {
        let status = match ping(ip, Some(Duration::from_secs(5)), None, None, None, None) {
            Ok(_) => true,
            Err(_) => false,
        };
        reset.export().unwrap();
        reset.set_direction(Direction::High).unwrap();
        power.export().unwrap();
        power.set_direction(Direction::High).unwrap();
        Server {
            reset,
            power,
            ip,
            status,
        }
    }
    fn start(&self) {
        press_pin(self.power, Duration::from_millis(500));
    }
    fn restart(&self) {
        press_pin(self.reset, Duration::from_millis(500));
    }
    fn shutdown(&self) {
        press_pin(self.power, Duration::from_secs(5));
    }

    fn _get_status(mut self) -> bool {
        self.status = match ping(
            self.ip,
            Some(Duration::from_secs(5)),
            None,
            None,
            None,
            None,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };
        self.status
    }
    fn ping(&self) -> bool {
        match ping(
            self.ip,
            Some(Duration::from_secs(5)),
            None,
            None,
            None,
            None,
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    fn get_up(&self) -> Result<(), &'static str> {
        if self.ping() == true {
            return Ok(());
        };
        self.start();
        sleep(Duration::from_secs(30));
        if self.ping() == true {
            println!("server startet after powering on");
            return Ok(());
        };
        self.restart();
        sleep(Duration::from_secs(30));
        if self.ping() == true {
            println!("server startet after rebooting");
            return Ok(());
        };
        self.shutdown();
        sleep(Duration::from_millis(500));
        self.start();
        sleep(Duration::from_secs(30));
        if self.ping() == true {
            println!("server startet after force reboot");
            return Ok(());
        } else {
            Err("couldnt start server")
        }
    }
}

fn press_pin(pin: Pin, dur: Duration) -> () {
    pin.set_value(0).unwrap();
    sleep(dur);
    pin.set_value(1).unwrap();
}

pub fn auto(server: &Server) {
    server.get_up().unwrap();
    sleep(Duration::from_secs(30));
}

pub fn manuel(server: &Server) -> i8 {
    loop {
        println!(
            "1. Start
2. reboot 
3. shutdwon
4. auto
5. exit
"
        );
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
            4 => return 1,
            5 => return -1,
            _ => continue,
        };
    }
}
