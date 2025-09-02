// hzoj-184
pub fn recursive_practice_184(n: u32) -> u64 {
    if n == 1 {
        1
    } else {
        (recursive_practice_184(n - 1) + 1) * 2
    }
}

// hzoj-186
pub fn recursive_practice_186(i: usize, arr: &[u32]) -> u32 {
    if i >= arr.len() {
        0
    } else {
        1 + recursive_practice_186(i + arr[i] as usize, arr)
    }
}
