pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut volume = 0;
    for (i, &curr_height) in height.iter().enumerate() {
        while let Some(&index) = stack.last() {
            if height[index] <= curr_height {
                stack.pop();
                if let Some(&top_index) = stack.last() {
                    let h = i32::min(height[top_index], curr_height) - height[index];
                    let w = i - (top_index + 1);
                    volume += h * (w as i32);
                }
                continue;
            }
            break;
        }
        stack.push(i);
    }
    volume
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result = trap(height);
    println!("result = {:?}", result);
}
