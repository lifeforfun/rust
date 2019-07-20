pub fn test() {
    let n = 40;
    println!("Fibonacci {}: {}", n, fibonacci(n, 1, 1));
}
// 需要尾递归并且值不能过大，否则会爆栈
fn fibonacci(times: u32, sum: u32, next: u32) -> u32 {
    if times < 2 {
        return sum;
    }
    return fibonacci(times - 1, next, next + sum);
}
