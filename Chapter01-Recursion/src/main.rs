mod recursion_example;

use recursion_example::*;
fn main() {
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
