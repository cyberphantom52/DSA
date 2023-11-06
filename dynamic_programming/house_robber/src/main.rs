pub fn rob_helper(index: usize, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
    if index == nums.len() - 1 || index == nums.len() - 2 {
        return nums[index];
    }

    if cache[index] == -1 {
        for i in (index + 2)..nums.len() {
            cache[index] = cache[index].max(nums[index] + rob_helper(i, nums, cache));
        }
    }

    cache[index]
}

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut cache = vec![-1; nums.len()];
    let result = i32::max(
        rob_helper(0, &nums, &mut cache),
        rob_helper(1, &nums, &mut cache),
    );
    0.max(result)
}

pub fn rob_tabulate(nums: Vec<i32>) -> i32 {
    let mut cache = nums.clone();
    for (i, &val) in nums.iter().enumerate().rev().skip(1) {
        for j in (i + 2)..nums.len() {
            cache[i] = cache[i].max(val + cache[j]);
        }
        cache[i] = i32::max(cache[i], cache[i + 1])
    }
    println!("{:?}", cache);
    cache[0]
}

fn main() {
    let nums = vec![
        114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110,
        236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228, 78,
        188, 67, 205, 94, 205, 169, 241, 202, 144, 240,
    ];
    // let result = rob(nums.clone());
    let result = rob_tabulate(nums);
    println!("result = {}", result);
}
