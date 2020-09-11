use regex::Regex;
use std::fmt;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum Scale {
    Celsius,
    Fahrenheit,
}

impl Scale {
    pub fn from_str(s: &str) -> Result<Scale, &'static str> {
        match s {
            "c" | "C" => Ok(Scale::Celsius),
            "f" | "F" => Ok(Scale::Fahrenheit),
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
    pub fn from_str(s: &str) -> Result<Temperature, &'static str> {
        let re = Regex::new(r"(^\d{1,3}[\.\d{1,3})]?)([c|f]$)").unwrap();
        let mut temp = Temperature {
            scale: Scale::Celsius,
            temp: 0.0,
        };
        if re.is_match(s) {
            for cap in re.captures_iter(s) {
                temp.temp = cap[1].parse::<f32>().unwrap();
                temp.scale = Scale::from_str(&cap[2]).unwrap();
            }
            Ok(temp)
        } else {
            Err("Not a valid input")
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}{}", &self.temp, &self.scale)
    }
}

fn main() {
    let mut s = String::new();
    print!("Input: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to parse input");

    let s = s.trim();

    let temp = Temperature::from_str(&s);
    match temp {
        Ok(o1) => {
            let c = convert(o1);
            match c {
                Ok(o2) => println!("{}", o2),
                Err(e2) => println!("{}", e2),
            }
        }
        Err(e1) => println!("Error: {}", e1),
    }
}

fn convert(t: Temperature) -> Result<Temperature, &'static str> {
    match t.scale {
        Scale::Celsius => {
            let new = Temperature {
                scale: Scale::Fahrenheit,
                temp: (t.temp * 1.8) + 32.0,
            };
            Ok(new)
        }
        Scale::Fahrenheit => {
            let new = Temperature {
                scale: Scale::Celsius,
                temp: ((t.temp - 32.0) * 5.0) / 9.0,
            };
            Ok(new)
        }
    }
}
