pub fn climb(n: usize, cache: &mut Vec<i32>) -> i32 {
    if n == 1 || n == 0 {
        return 1;
    }

    if cache[n] == -1 {
        cache[n] = climb(n - 1, cache) + climb(n - 2, cache);
    }
    
    return cache[n];
}

pub fn climb_space_opt(n: usize) -> i32 {
    let mut prev: [i32; 2] = [1, 1];
    let mut next = 0;
    for _ in 2..=n {
        next = prev[1] + prev[0];
        prev[0] = prev[1];
        prev[1] = next;
    }
    next
}

fn main() {
    let n = 10;
    // let mut cache = vec![-1; n + 1];
    // let result = climb(n, &mut cache);
    let result = climb_space_opt(n);
    println!("result = {}", result);
}
