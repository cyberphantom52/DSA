pub fn xor_of_n(n: i32) -> i32 {
    match n % 4 {
        0 => n,
        1 => 1,
        2 => n + 1,
        3 => 0,
        _ => unreachable!(),
    }
}

pub fn first_set_bit(n: i32) -> i32 {
    n & !(n - 1)
}

pub fn missing_and_repeating_number(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let xor = nums.iter().fold(xor_of_n(n), |acc, &x| acc ^ x);
    let xor_fb = first_set_bit(xor);
    
    let (mut a, mut b) = nums.iter().fold((0, 0), |(r, m), &x| {
        if first_set_bit(x) == xor_fb {
            (r ^ x, m)
        } else {
            (r, m ^ x)
        }
    });

    for i in 1..=n {
        if first_set_bit(i) == xor_fb {
            a ^= i;
        } else {
            b ^= i;
        }
    }

    for &x in &nums {
        if x == a {
            return vec![a, b];
        }
    }
    
    vec![b, a]
}

fn main() {
    let nums = vec![3, 1, 2, 5, 3];
    let result = missing_and_repeating_number(nums);
    println!("{:?}", result);
}
