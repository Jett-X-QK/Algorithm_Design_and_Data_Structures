mod recursion_example;
mod recursion_hzoj;

use proconio::input;
use recursion_example::*;
use recursion_hzoj::*;
use std::io;
fn main() {
    test03();
}

fn _test01() {
    let n = 5;
    let sum = sum_odd_numbers(n);
    println!("前 {} 项奇数的累加和为 {}", n, sum);

    let n = 5;
    let m = 2;
    let result = combinations(n, m);
    println!("从 {} 个球中 {} 个有 {} 取法", n, m, result);

    for i in 0..20 {
        println!("F({}) = {}", i, fibonacci(i));
    }

    let a = power(5, 2);
    println!("{}", a);

    let b = sum_of_digits(12345);
    println!("{}", b);

    let mut chars: Vec<char> = "abcdef".chars().collect();
    let len = chars.len();
    reverse_recursive(&mut chars, 0, len - 1);
    let reversed: String = chars.into_iter().collect();
    println!("{}", reversed);

    let strings = String::from("hello world");
    let len = strlen_recursive(&strings);
    println!("strings len {}", len);

    let n = 5;
    let num01 = climb_naive(n);
    let num02 = climb_naive_update(n);
    println!("{}", num01);
    println!("{}", num02);
}

fn test02() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");

    let n: u32 = input.trim().parse().expect("error");

    println!("{}", recursive_practice_184(n));
}

fn test03() {
    input! {
        n:usize,
        arr:[u32;n],
    }
    let res = recursive_practice_186(0, &arr);
    println!("{}", res);
}
