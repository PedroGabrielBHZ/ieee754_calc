/*
This program implements a simple CLI tool that converts a hexadecimal number
in IEEE 754 format to its decimal representation. The program accepts both 32-bit
and 64-bit floating point numbers.
 */

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ieee754_calc <hexadecimal number>");
        process::exit(1);
    }

    // parse hexadecimal number
    let hex = &args[1];
    println!("Hexadecimal number:\t{}", hex);

    // check if hex number is valid
    let length = hex.len();
    if length != 8 && length != 16 {
        println!("Invalid hexadecimal number");
        process::exit(1);
    }

    // convert hex number to binary, display result in scientific notation
    if length == 8 {
        let bits = match u32::from_str_radix(hex, 16) {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        };
        let result = f32::from_bits(bits);
        println!("{}", scientific_notation_f32(result));
    } else {
        let bits = match u64::from_str_radix(hex, 16) {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        };

        let result = f64::from_bits(bits);
        println!("{}", scientific_notation_f64(result));
    }
}

/*
Auxiliary functions
 */

// Function to convert f32 decimal number into scientific notation
fn scientific_notation_f32(number: f32) -> String {
    let mut exponent = 0;
    let mut mantissa = number;
    while mantissa >= 10.0 {
        mantissa /= 10.0;
        exponent += 1;
    }
    while mantissa > -1.0 {
        mantissa *= 10.0;
        exponent -= 1;
    }
    format!("Decimal number:\t\t{}e{}", mantissa, exponent)
}

// Function to convert f64 decimal number into scientific notation
fn scientific_notation_f64(number: f64) -> String {
    let mut exponent = 0;
    let mut mantissa = number;
    while mantissa >= 10.0 {
        mantissa /= 10.0;
        exponent += 1;
    }
    while mantissa > -1.0 {
        mantissa *= 10.0;
        exponent -= 1;
    }
    format!("Decimal number:\t\t{}e{}", mantissa, exponent)
}
