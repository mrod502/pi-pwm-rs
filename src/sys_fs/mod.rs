use std::{fmt::Error, fs::read_to_string, path::Path};

use crate::pwm::{Polarity, PwmRead};

#[derive(Debug, Clone)]
pub struct SysFsPwm {
    path: String,
}

impl SysFsPwm {
    pub fn new(chip: u8) -> Result<Self, ()> {
        Ok(Self {
            path: format!("/sys/class/pwm/pwmchip{}", chip),
        })
    }

    fn get_path(&self) -> &Path {
        Path::new(&self.path)
    }

    fn read(&self, channel: u8, segment: &str) -> Result<usize, std::io::Error> {
        println!("getting path");
        let f = read_to_string(
            self.get_path()
                .join(format!("pwm{}", channel))
                .join(segment),
        )?;
        println!("path:{}", f);

        Ok(usize::from_str_radix(&f, 10).unwrap_or(0))
    }
}

impl PwmRead for SysFsPwm {
    type Error = ();
    fn get_duty_cycle(&self, interface: u8) -> Result<usize, Self::Error> {
        Err(())
    }
    fn get_enabled(&self, interface: u8) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn get_period(&self, interface: u8) -> Result<usize, Self::Error> {
        Err(())
    }
    fn get_polarity(&self, interface: u8) -> Result<Polarity, Self::Error> {
        Ok(Polarity::Normal)
    }
}
#[cfg(test)]
mod test {
    use crate::sys_fs::SysFsPwm;

    #[test]
    fn test_read() {
        let sfs = SysFsPwm::new(0).unwrap();
        println!("{:?}", sfs);
        println!("result:{}", sfs.read(1, "duty_cycle").unwrap());
    }
}
