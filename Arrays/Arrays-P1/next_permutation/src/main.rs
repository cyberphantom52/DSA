pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut peak = 0;
    let size = nums.len();
    for i in (0..(size - 1)).rev() {
        if nums[i] < nums[i + 1] {
            peak = i + 1;
            break;
        }
    }

    if peak == 0 {
        nums.reverse();
        return;
    }

    for i in (0..size).rev() {
        if nums[i] > nums[peak - 1] {
            nums.swap(i, peak - 1);
            break;
        }
    }
    
    nums[peak..size].reverse();
}

fn main() {
    // let mut nums: Vec<i32> = vec![3, 2, 1];
    // let mut nums: Vec<i32> = vec![1, 3, 2]; // [2, 1, 3]
    // let mut nums: Vec<i32> = vec![4,2,0,2,3,2,0]; 
    let mut nums = vec![5,4,7,5,3,2];
                              // [4,2,0,3,0,2,2]
    // let mut nums: Vec<i32> = vec![4, 9, 7, 1, 13, 12, 3, 8, 2, 10, 11, 6, 15, 15, 14];
    
    next_permutation(&mut nums);
    println!("{:?}", nums);
}
