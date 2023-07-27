fn my_pow(x: f64, n: i32) -> f64 {
    let mut result = 1 as f64;
    let (mut x, mut n) = (x, n as i64);

    if n.is_negative() {
        n = n.abs();
        x = 1.0 / x;
    }
    
    while n != 0 {
        if n & 1 == 1 {
            result *= x;
        }
        n >>= 1;
        x *= x;
    }

    result
}

fn main() {
    let x = 2.00000;
    let n = -2147483648;

    println!("{}^{} = {}", x, n, my_pow(x, n));
}
