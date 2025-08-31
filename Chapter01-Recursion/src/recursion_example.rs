// Use recursion to calculate the cumulative sum of the first n odd numbers
pub fn sum_odd_numbers(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        sum_odd_numbers(n - 1) + (2 * n - 1)
    }
}

// Among n balls, select m balls (without replacement),
// and find out how many different ways there are to do this.
pub fn combinations(n: u32, m: u32) -> u32 {
    if m > n {
        return 0;
    }
    if m == 0 || m == n {
        return 1;
    }
    combinations(n - 1, m - 1) + combinations(n - 1, m)
}

// Calculate the Fibonacci sequence
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Calculate the k-th power of n
pub fn power(n: i64, k: i32) -> i64 {
    if k == 0 { 1 } else { power(n, k - 1) * n }
}

//
pub fn sum_of_digits(n: u64) -> u64 {
    if n < 10 {
        n
    } else {
        (n % 10) + sum_of_digits(n / 10)
    }
}

// Element reversal
pub fn reverse_recursive(chars: &mut [char], left: usize, right: usize) {
    if left >= right {
        return;
    }
    chars.swap(left, right);
    reverse_recursive(chars, left + 1, right - 1);
}

// Implement strlen

pub fn strlen_recursive(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        1 + strlen_recursive(&s[1..])
    }
}


// 树老师爬楼梯，他可以每次走 1 级或者 2 级，输入楼梯的级数，求不同的走法数。
pub fn climb_naive(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => climb_naive(n - 1) + climb_naive(n - 2),
    }
}

// 树老师爬楼梯，他可以每次走 1 级、2 级或者 3 级，输入楼梯的级数，求不同的走法数。

pub fn climb_naive_update(n:u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => climb_naive_update(n-1)+ climb_naive_update(n-2) + climb_naive_update(n-3),
    }
}