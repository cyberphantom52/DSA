pub fn merge(left: Vec<i32>, right: Vec<i32>, count: &mut i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let (mut i, mut j) = (0, 0);
    loop {
        if i == left.len() {
            result.extend(right[j..].iter());
            break;
        } else if j == right.len() {
            result.extend(left[i..].iter());
            break;
        }

        if left[i] > right[j] {
            *count += left.len() as i32 - i as i32;
            result.push(right[j]);  
            j += 1;
        } else {
            result.push(left[i]);
            i += 1;
        }
    }
    result
}

fn merge_sort(mut input: Vec<i32>, count: &mut i32) -> Vec<i32> {
    if input.len() == 1 {
        return input;
    }
    
    let mid = input.len() / 2;
    let left = merge_sort(input.drain(..mid).collect(), count);
    let right = merge_sort(input, count);
    merge(left, right, count)
}

// Time Complexity: O(nlogn)
// Space Complexity: O(n)
// inversion - for (i, j), i < j and nums[i] > nums[j]
pub fn number_of_inversions(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let result = merge_sort(nums, &mut count);
    println!("{:?}", result);
    count
}

fn main() {
    // let nums = vec![2, 5, 1, 3, 4]; // 4
    let nums = vec![5, 3, 2, 1, 4]; // 7
    let result = number_of_inversions(nums);
    println!("result = {}", result);
}
