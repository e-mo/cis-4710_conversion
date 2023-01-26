//! # Assignment 1 - Conversion  
//! Evan Morse  
//! emorse8686@gmail.com  
//! CIS-4710
//! last edited: 1.26.23  
//!
//! ### Examples
//! ```command-line
//! @> conversion 221 
//! 221.0F => 105.0C
//! 
//! @> conversion 0.0 C
//! 0.0C => 32.0F
//!
//! @> conversion
//! Enter a termperature (f32) to convert.
//! Syntax: TEMP [C|F]
//! $> 32.0 F
//! 32.0F => 0.0C
//! ```

use std::{io::{self, Write}, env};

// Temp unit constants
/// Fahrenheit 
const UNIT_F: &str = "F";
/// Celcius
const UNIT_C: &str = "C";
/// Absolute zero in F
const LIMIT_MIN_F: f32 = -469.67; 
/// Absolute zero in C
const LIMIT_MIN_C: f32 = -273.15; 

/// Takes temperature input from either the command line or 
/// prompted user input if no command line arguments are provided.
///
/// # Argument Syntax 
/// `TEMP [C|F]`   
/// Any extra arguments will be ignored. If no unit argument is provided,
/// a default will be used.
///
/// ### Argument Examples
/// `0.0 C` <- Will be converted to F   
/// `32.0 F` <- Will be converted to C   
fn main() -> io::Result<()> {
    let arg_iter = env::args();
    let default_unit = UNIT_F;

    let mut input = String::new();
    if arg_iter.len() > 1 { // Use command line arguments by default
        for arg in arg_iter.skip(1) {
            input.push_str(&format!("{arg} "));
        }
    } else { // Else prompt for input
        println!("Enter a temperature (f32) to convert.\n\
                  Syntax: TEMP [C|F]");
        print!("$> ");
        io::stdout().flush().expect("Error flushing stdout.");
        io::stdin().read_line(&mut input)?;
    }

    if !input.is_empty() {
        // Parse input
        let (temp, starting_unit, conversion_unit) =
                match get_inputs(&input, default_unit) {
            Some(t) => t,
            None => std::process::exit(1),
        };

        // Convert temp
        let conversion: f32 = convert(temp, conversion_unit);

        // Desplay output
        println!{"{temp:.1}{starting_unit} => {conversion:.1}{conversion_unit}"}

    }

    Ok(())
}
/// Parses/Validates input String.
///  
/// # Arguments
/// * `input` - `&str` containing user input.
/// * `def_unit` - one of the unit constants (UNIT_F/UNIT_C) to use as a default
/// temperature unit.
///
/// # Return
/// Returns Option<(f32, &str, &str)> on success.
/// * 0:`f32` - temperature input from user converted to `f32` 
/// * 1: `&str` - unit constant for input temperature
/// * 2: `&str` - unit constant for conversation temperature
///   
/// Returns `None` on failure and prints relevant error message to stdout. 
fn get_inputs<'a>(input: &'a str, def_unit: &'a str) -> Option<(f32, &'a str, &'a str)>  {
    // Split input into tokens along whitespace 
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() < 1 { return None; } // No args
    // args.len() is > 0 so I feel safe expecting here
    let temp: &str = args.first().expect("Expected first argument.");
    let temp: f32 = match temp.parse() {
        Ok(i) => i,
        Err(e) => {
            println!("Invalid temperature \"{temp}\": {e}");
            return None;
        }
    };

    // Use second arguement as starting unit (if valid), otherwise
    // default value is used
    let starting_unit: &str;
    if args.len() > 1 {
        starting_unit = match args.get(1).expect("Expected second argument.")
                .to_uppercase().as_str() {
            UNIT_F => UNIT_F, 
            UNIT_C => UNIT_C, 
            _ => {
                println!("Invalid unit -> using default unit \"{def_unit}\".");
                def_unit
            }
        }
    } else {
        starting_unit = def_unit;
    }

    // Ensure temperature is valid i.e. above absolute zero. 
    match starting_unit {
        UNIT_F => {
            if temp < LIMIT_MIN_F {
                println!("Invalid temperature: below absolute zero!");
            }
        }
        UNIT_C => {
            if temp < LIMIT_MIN_C {
                println!("Invalid temperature: below absolute zero!");
            }
        }
        _ => unreachable!(),
    }

    let conversion_unit: &str = match starting_unit {
        UNIT_F => UNIT_C,
        UNIT_C => UNIT_F,
        _ => unreachable!(),
    };

    Some((temp, starting_unit, conversion_unit)) 
}

/// Converts temperature to F or C depending on desired unit.
/// 
/// # Arguments
/// * `temp: f32` - numeric temperature to convert. 
/// * `desired_unit: &str` - unit constant (UNIT_F|UNIT_C) of desired conversion.
fn convert(temp: f32, desired_unit: &str) -> f32 {
    match desired_unit {
        UNIT_F => convert_to_f(temp),
        UNIT_C => convert_to_c(temp),
        _ => panic!("convert: Unexpected value for temp.")
    }
}

/// Converts temperature from C to F
fn convert_to_f(temp_c: f32) -> f32 {
    temp_c * (9.0/5.0) + 32.0 
}

/// Converts temperature from F to C
fn convert_to_c(temp_f: f32) -> f32 {
    (temp_f - 32.0) * (5.0/9.0)
}

#[cfg(test)]
mod unit_tests {
    use super::{convert, get_inputs, UNIT_F, UNIT_C};
     
    // get_input using default unit
    #[test]
    fn get_inputs_default() {
        let default_unit = UNIT_F;
        let input = String::from("32");
        let (temp, starting_unit, conversion_unit) =
                match get_inputs(&input, default_unit) {
            Some(t) => t,
            None => std::process::exit(1),
        };
        assert!(temp == 32.0);
        assert!(starting_unit == UNIT_F);
        assert!(conversion_unit == UNIT_C);
    }

    // get_input with user provided unit
    #[test]
    fn get_inputs_no_default() {
        let default_unit = UNIT_F;
        let input = String::from("0 C");
        let (temp, starting_unit, conversion_unit) =
                match get_inputs(&input, default_unit) {
            Some(t) => t,
            None => std::process::exit(1),
        };
        assert!(temp == 0.0);
        assert!(starting_unit == UNIT_C);
        assert!(conversion_unit == UNIT_F);
    }

    // Invalid input String
    #[test]
    #[should_panic]
    fn get_inputs_fail() {
        let default_unit = UNIT_F;
        let input = String::from("32C F sdgh");
        let (_, _, _) = match get_inputs(&input, default_unit) {
            Some(t) => t,
            None => panic!("Invalid input string,"),
        };
    }

    // 0C -> 32F
    #[test]
    fn convert_c2f() {
        let temp: f32 = 0.0;
        let result = convert(temp, UNIT_F);     
        assert!(result == 32.0);
    }

    // 32F -> 0C
    #[test]
    fn convert_f2c() {
        let temp: f32 = 32.0;
        let result = convert(temp, UNIT_C);     
        assert!(result == 0.0);
    }
}
