pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[index] {
            index += 1;
            nums[index] = nums[i];
        }
    }
    
    (index + 1) as i32
}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = remove_duplicates(&mut nums);
    println!("array = {:?}", nums);
    println!("result = {}", result);
}
