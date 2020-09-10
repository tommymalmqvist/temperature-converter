use regex::Regex;
use std::fmt;
use std::io;

#[derive(Debug)]
enum Scale {
    Celsius,
    Fahrenheit,
}

impl Scale {
    pub fn from_str(s: &str) -> Result<Scale, &'static str> {
        match s {
            "c" => Ok(Scale::Celsius),
            "f" => Ok(Scale::Fahrenheit),
            "C" => Ok(Scale::Celsius),
            "F" => Ok(Scale::Fahrenheit),
            _ => Err("Failed to parse"),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Scale::Celsius => 'c',
            Scale::Fahrenheit => 'f',
        };
        write!(f, "{}", printable)
    }
}

struct Temperature {
    scale: Scale,
    temp: f32,
}

impl Temperature {
    pub fn from_str(s: &str) -> Result<Temperature, String> {
        let re = Regex::new(r"(^\d{1,3}\.?\d{1,3})([c|f]$)").unwrap();
        let mut temp = Temperature {
            scale: Scale::Celsius,
            temp: 0.0,
        };
        for cap in re.captures_iter(s) {
            temp.temp = cap[1].parse::<f32>().unwrap();
            temp.scale = Scale::from_str(&cap[2]).unwrap();
        }
        Ok(temp)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", &self.temp, &self.scale)
    }
}

fn main() {
    let mut s = String::new();
    println!("Input:");
    io::stdin()
        .read_line(&mut s)
        .expect("failed to parse input");

    let s = s.trim();

    let temp = Temperature::from_str(&s);
    if temp.is_ok() {
        // begin converting
        println!("{}", temp.unwrap());
    }
}

fn convert(t: Temperature) -> Result<Temperature, &'static str> {
    Err("Fail!")
}
