use std::fmt::Debug;

pub enum Polarity {
    Normal,
}

pub trait PwmRead {
    type Error: Sized + Debug;
    fn get_period(&self, interface: u8) -> Result<usize, Self::Error>;
    fn get_duty_cycle(&self, interface: u8) -> Result<usize, Self::Error>;
    fn get_polarity(&self, interface: u8) -> Result<Polarity, Self::Error>;
    fn get_enabled(&self, interface: u8) -> Result<bool, Self::Error>;
}
pub trait PwmWrite {
    type Error: Sized + Debug;
    fn set_period(&self, interface: u8) -> Result<usize, Self::Error>;
    fn set_duty_cycle(&self, interface: u8) -> Result<usize, Self::Error>;
    fn set_polarity(&self, interface: u8) -> Result<Polarity, Self::Error>;
    fn set_enabled(&self, interface: u8) -> Result<bool, Self::Error>;
}

pub trait Pwm: PwmRead + PwmWrite {}
