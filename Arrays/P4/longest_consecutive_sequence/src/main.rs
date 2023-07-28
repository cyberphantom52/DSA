pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<_> = nums.into_iter().collect();

    set.iter() // iterate over the set
        .filter(|n| !set.contains(&(*n - 1)))   // filter out all elements for which `element - 1` is not the set i.e the previous sequence breaks and a new sequence starts
        .map(|n| (*n..).take_while(|next| set.contains(next)).count())  // loop over all elements (n, n + 1 ... inf) count how many of them are in the set 
        .max()  // get the maximum count
        .unwrap_or_default() as i32
}

fn main() {
    // let nums = vec![100, 4, 200, 1, 3, 2]; // 4
    let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]; // 9
    println!("Longest subsequence: {}", longest_consecutive(nums));
}
