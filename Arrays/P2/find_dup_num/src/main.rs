pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let (mut slow, mut fast) = (nums[0] as usize, nums[0] as usize);
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }
    fast = nums[0] as usize;
    while slow != fast {
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
    }
    slow as i32
}

fn main() {
    let nums = vec![1,3,4,2,2];
    println!("The duplicate number is {}", find_duplicate(nums));
}
