// Problem Link:
// https://www.hackerrank.com/challenges/time-conversion/problem?h_r=next-challenge&h_v=zen

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let numero: i32 = s[0..2].parse().unwrap(); // comparar
    let AorP: char = s.chars().nth(8).unwrap(); // letra A o P
    let mut nuevaString: String = String::from("");
    if AorP == 'P' {
        if numero < 12 {
            nuevaString.push_str((numero+12).to_string().as_str());
        } else {
            nuevaString.push_str(&s[0..2]);
        }
    } else if AorP == 'A' {
        if numero == 12 {
            nuevaString.push_str("00");
        } else {
            nuevaString.push_str(&s[0..2]);
        }
    }
    nuevaString.push_str(&s[2..8]);
    return nuevaString;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}