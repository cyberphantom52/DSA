fn cost(i: usize, j: usize, heights: &Vec<i32>) -> i32 {
    (heights[i] - heights[j]).abs()
}

fn minimize(n: usize, k: usize, heights: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
    if n == 0 || n == 1 {
        return cost(0, n, heights);
    }

    if cache[n] == -1 {
        cache[n] = i32::MAX;
        for i in 1..=usize::min(k, n) {
            cache[n] = i32::min(
                cache[n],
                minimize(n - i, k, heights, cache) + cost(n - i, n, heights),
            );
        }
    }

    cache[n]
}

fn minimize_cost(k: usize, heights: &Vec<i32>) -> i32 {
    let n = heights.len();
    let mut cache = vec![-1; n];
    minimize(n - 1, k, heights, &mut cache)
}

fn minimize_cost_space_opt(k: usize, heights: &Vec<i32>) -> i32 {
    let mut prev = vec![i32::MAX; k];
    prev[0] = 0;
    let mut curr = 0;
    for i in 1..heights.len() {
        curr = i32::MAX;
        let target = usize::min(i, k);
        for j in 0..target {
            curr = i32::min(curr, prev[j] + cost(i, i - (target - j), heights));
        }
        if k > i {
            prev[i] = curr;
        } else {
            for l in 1..k {
                prev[l - 1] = prev[l];
            }
            prev[k - 1] = curr;
        }
    }
    curr
}

fn main() {
    let k = 3;
    let heights = vec![10, 40, 50, 20, 60];
    // let k = 2;
    // let heights = vec![10, 40, 30, 10];
    // let result = minimize_cost(k, &heights);
    let result = minimize_cost_space_opt(k, &heights);
    println!("result = {}", result);
}
