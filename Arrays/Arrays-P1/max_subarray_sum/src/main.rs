pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut global_max, mut local_max) = (nums[0], nums[0]);
    let size = nums.len();

    for i in 1..size {
        local_max = i32::max(nums[i], local_max + nums[i]);
        global_max = i32::max(global_max, local_max);
    }
    global_max
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    
    println!("max_subarray_sum : {}", max_sub_array(nums));
}
