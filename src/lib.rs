use std::thread::sleep;
use ping::ping;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};
use std::net::IpAddr;

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
    pub fn start(&self) {
        press_pin(self.power, Duration::from_millis(500));
    }
    pub fn restart(&self) {
        press_pin(self.reset, Duration::from_millis(500));
    }
    pub fn shutdown(&self) {
        press_pin(self.power, Duration::from_secs(5));
    }
    
    pub fn get_status(mut self) -> bool {
        self.status = match ping(self.ip, Some(Duration::from_secs(5)), None, None, None, None) {
            Ok(_) => true, 
            Err(_) => false,
        };
        self.status
    }
    pub fn ping(&self) -> bool {
        match ping(self.ip, Some(Duration::from_secs(5)), None, None, None, None) {
            Ok(_) => true, 
            Err(_) => false,
        }
    }
    pub fn get_up(&self) -> Result<(), &'static str> {
        if self.ping() == true {
            return Ok(())
        };
        self.start();
        sleep(Duration::from_secs(10));
        if self.ping() == true {
            return Ok(())
        };
        self.restart();
        if self.ping() == true {
            return Ok(())
        };
        self.shutdown();
        sleep(Duration::from_millis(500));
        self.start();
        if self.ping() == true {
            return Ok(())
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
