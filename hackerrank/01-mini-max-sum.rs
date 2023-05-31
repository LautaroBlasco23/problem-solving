// Problem link:
// https://www.hackerrank.com/challenges/mini-max-sum/problem

use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut sum: i64 = 0;
    let mut min: i32 = arr[0];
    let mut max: i32 = arr[arr.len()-1];
    for i in 0..arr.len() {
        sum += arr[i] as i64;
        if arr[i] < min {
            min = arr[i];
        } else if arr[i] > max {
            max = arr[i];
        }
    }
    print!("{} {}", (sum-max as i64), (sum-min as i64));
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}