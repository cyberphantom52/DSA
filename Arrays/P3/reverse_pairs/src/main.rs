pub fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
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

        if left[i] >= right[j] {
            result.push(right[j]);  
            j += 1;
        } else {
            result.push(left[i]);
            i += 1;
        }
    }
    result
}

pub fn count_pairs(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut rt = 0;
    let mut count = 0;
    for (index, num) in left.iter().enumerate() {
        while rt < right.len() && num > &right[rt].saturating_mul(2) {
            count += (left.len() - index) as i32;
            rt += 1;
        }
    }
    count
}

fn merge_sort(mut input: Vec<i32>, count: &mut i32) -> Vec<i32> {
    if input.len() == 1 {
        return input;
    }
    
    let mid = input.len() / 2;
    let left = merge_sort(input.drain(..mid).collect(), count);
    let right = merge_sort(input, count);
    *count += count_pairs(&left, &right);
    merge(left, right)
}

pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    merge_sort(nums, &mut count);
    count
}

fn main() {
    // let nums = vec![1, 3, 2, 3, 1]; // 2
    let nums = vec![2, 4, 3, 5, 1]; // 3
    let result = reverse_pairs(nums);

    println!("result = {}", result);
}
