pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut m, mut n, mut index) = (m as usize, n as usize, nums1.len());

    while index > 0 && n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1.swap(index - 1, m - 1);
            m -=  1;
        } else {
            std::mem::swap(&mut nums1[index - 1], &mut nums2[n - 1]);
            n -= 1;
        }
        index -= 1;
    }
}

fn main() {
    // let mut nums1 = vec![1,2,3,0,0,0];
    // let mut nums2 = vec![2,5,6];
    // let (m, n) = (3, 3);
    let mut nums1 = vec![4,5,6,0,0,0];
    let mut nums2 = vec![1,2,3];
    let (m, n) = (3, 3);
    merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}
