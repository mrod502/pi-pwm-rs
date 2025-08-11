use clap::Parser;

use crate::{pwm::PwmRead, sys_fs::SysFsPwm};

mod pwm;
mod sys_fs;

#[derive(clap::Parser)]
pub struct App {
    pub chip: u8,
    pub channel: u8,
}

fn main() {
    let app = App::parse();
    let s = &SysFsPwm::new(app.chip).unwrap();
    let x = s.get_enabled(app.channel).unwrap();
    println!("enabled: {}", x);
    let x = s.get_period(app.channel).unwrap();
    println!("period: {}ns", x);
    let x = s.get_duty_cycle(app.channel).unwrap();
    println!("duty cycle: {}ns", x);
}
