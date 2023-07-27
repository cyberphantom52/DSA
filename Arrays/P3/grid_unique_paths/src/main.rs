// Combinatorics
// m + n - 2 choose m - 1
// m + n - 2 choose n - 1
fn unique_paths(m: i32, n: i32) -> i32 {
    let mut result: f64 = 1.0;
    for i in 1..=(m - 1) {
        result = result * (n - 1 + i) as f64 / i as f64;

    }
    result as i32
}

// Recursive
fn recursive_paths(i: i32, j: i32, m: i32, n: i32) -> i32 {
    if i < 0 || i >= m || j < 0 || j >= n {
        return 0;
    } else if i == (m - 1) && j == (n - 1) {
        return 1;
    }
    recursive_paths(i + 1, j, m, n) + recursive_paths(i, j + 1, m, n)
}


// Dynamic Programming
fn dp_paths(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, m: i32, n: i32, cache_stats: &mut (u32, u32, u32)) -> i32 {
    if i < 0 || i >= m || j < 0 || j >= n {
        return 0;
    }
    
    cache_stats.0 += 1;
    if grid[i as usize][j as usize] != -1 {
        cache_stats.1 += 1;
        return grid[i as usize][j as usize];
    }
    cache_stats.2 += 1;

    if i == (m - 1) && j == (n - 1) {
        return 1;
    }
    grid[i as usize][j as usize] = dp_paths(grid, i + 1, j, m, n, cache_stats) + dp_paths(grid, i, j + 1, m, n, cache_stats);
    grid[i as usize][j as usize]
}

fn main() {
    let (rows, cols) = (15, 15);
    let mut time = std::time::Instant::now();
    println!("Unique Paths: {}", unique_paths(rows, cols));
    println!("Time: {}ms", time.elapsed().as_millis());
    
    time = std::time::Instant::now();
    println!("Recursive Paths: {}", recursive_paths(0, 0, rows, cols));
    println!("Time: {}ms", time.elapsed().as_millis());

    // (accesses, hits, misses)
    let mut cache_stats = (0, 0, 0);
    let mut grid = vec![vec![-1; cols as usize]; rows as usize];
    
    time = std::time::Instant::now();
    println!("DP Paths: {}", dp_paths(&mut grid, 0, 0, rows, cols, &mut cache_stats));
    println!("Time: {}ms", time.elapsed().as_millis());

    println!("Cache Accesses: {}", cache_stats.0);
    println!("Cache Hits: {:.2}%", cache_stats.1 as f64 / cache_stats.0 as f64 * 100.0);
    println!("Cache Misses: {:.2}%", cache_stats.2 as f64 / cache_stats.0 as f64 * 100.0);
}