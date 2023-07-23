// Original Solution
pub fn sort_colors(nums: &mut Vec<i32>) {
    let size = nums.len();
    let (mut high, mut low) = (size - 1, 0);

    if size == 1 {
        return;
    }

    for i in 0..size {
        while high > 0    && nums[high] == 2 {high -= 1;}
        while low  < size && nums[low]  == 0 {low += 1;}        

        if nums[i] == 2 && i < high {
            nums.swap(i, high);
        }

        if nums[size - i - 1] == 0 && size - i - 1 > low {
            nums.swap(size - i - 1, low);
        }
    }
}

// Dutch National Flag Algorithm
pub fn sort_colors2(nums: &mut Vec<i32>) {
    let size = nums.len();
    let (mut low, mut mid, mut high) = (0, 0, size - 1);

    while mid <= high && high != 0 {
        match nums[mid] {
            0 => {
                nums.swap(mid ,low);
                low += 1;
                mid += 1;
            },
            2 => {
                nums.swap(mid ,high);
                high -= 1;
            },
            _ => {
                mid += 1;
            }
        }
    }
}
fn main() {
    // let mut nums = vec![2,0,2,1,1,0];   
    // let mut nums = vec![1, 1];    
    // let mut nums = vec![1, 2, 0];    
    // let mut nums = vec![0];
    // let mut nums = vec![1];
    let mut nums = vec![2, 2];
    // sort_colors(&mut nums);
    sort_colors2(&mut nums);

    println!("Sort Colors: {:?}", nums);
}
