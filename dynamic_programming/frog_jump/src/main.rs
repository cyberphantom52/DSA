pub fn cost(i: usize, j: usize, heights: &Vec<i32>) -> i32 {
    (heights[i] - heights[j]).abs()
}

pub fn frog_jump(n: usize, heights: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return cost(0, 1, heights);
    }

    if cache[n] == -1 {
        cache[n] = i32::min(
            frog_jump(n - 1, heights, cache) + cost(n, n - 1, heights),
            frog_jump(n - 2, heights, cache) + cost(n, n - 2, heights),
        );
    }

    cache[n]
}

pub fn frog_jump_space_opt(heights: &Vec<i32>) -> i32 {
    let mut cache = vec![0, cost(0, 1, heights)];
    let mut curr = 0;
    for i in 2..heights.len() {
        curr = i32::min(
            cache[0] + cost(i, i - 2, heights),
            cache[1] + cost(i, i - 1, heights),
        );

        cache[0] = cache[1];
        cache[1] = curr;

    }

    curr
}

fn main() {
    let heights = vec![10, 20, 30, 10];
    // let mut cache = vec![-1; heights.len() + 1];
    // let result = frog_jump(heights.len() - 1, &heights, &mut cache);
    let result = frog_jump_space_opt(&heights);
    println!("result = {}", result);
}
