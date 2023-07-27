pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (candidate, _) = nums.iter().fold((0, 0), |(c, v), &x|{
        if v == 0 {
            (x, 1)
        } else if c == x {
            (c, v + 1)
        } else {
            (c , v - 1)
        }
    });
    candidate
}

fn main() {
    let nums = vec![2,2,1,1,1,2,2];
    // let nums = vec![6,5,5];
    // let nums = vec![3, 3, 4];
    let result = majority_element(nums);
    
    println!("result: {:?}", result);
}
