pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    let mut result: Vec<Vec<i32>> = Vec::new();
    nums.sort_unstable();
    let len = nums.len();
    for i in 0..len {
        // Check for duplicates
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        for j in (i + 1)..len {
            // Check for duplicates
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let (mut left, mut right) = (j + 1, len - 1);
            while left < right {
                match (nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64).cmp(&(target as i64)) {
                    Ordering::Greater => {
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        right -= 1;
                    }
                    Ordering::Less => {
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        left += 1;
                    }
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        right -= 1;
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        left += 1;
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let mut nums = vec![1000000000,1000000000,1000000000,1000000000];
    let mut target = -294967296;
    println!("{:?}", four_sum(nums, target));
    nums = vec![1, 0, -1, 0, -2, 2];
    target = 0;
    println!("{:?}", four_sum(nums, target));
    nums = vec![2, 2, 2, 2, 2];
    target = 8;
    println!("{:?}", four_sum(nums, target));
}
