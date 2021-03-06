use regex::Regex;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug)]
enum Scale {
    Celsius,
    Fahrenheit,
}

impl FromStr for Scale {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Scale, &'static str> {
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

#[derive(Debug)]
struct Temperature {
    scale: Scale,
    temp: f32,
}

impl FromStr for Temperature {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Temperature, &'static str> {
        let re = Regex::new(r"(^\d{1,3})(\.\d{1,3})?([c|f]$)").unwrap();
        let mut temp = Temperature {
            scale: Scale::Celsius,
            temp: 0.0,
        };
        if re.is_match(s) {
            for cap in re.captures_iter(s) {
                temp.temp = cap[1].parse::<f32>().unwrap();
                temp.scale = Scale::from_str(&cap[3]).unwrap();
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    print!("Input: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to parse input");

    let s = s.trim();

    let temp = Temperature::from_str(&s)?;
    let convert = convert(temp)?;
    println!("{}", convert);
    Ok(())
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
