#[inline(always)]
pub fn merge_interval(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    vec![i32::min(a[0], b[0]), i32::max(a[1], b[1])]
}

#[inline(always)]
pub fn mergable(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    (b[0] >= a[0] && b[0] <= a[1]) || (b[1] >= a[0] && b[1] <= a[1]) || (a[0] >= b[0] && a[1] <= b[1])
}

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let num_intervals = intervals.len();
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_intervals);

    for i in 0..num_intervals {
        if result.is_empty() {
            result.push(intervals[i].clone());
            continue;
        }

        if let Some(interval) = result.last_mut() {
            if mergable(interval, &intervals[i]) {
                *interval = merge_interval(interval, &intervals[i]);
            } else {
                result.push(intervals[i].clone());
            }
        }
    }
    result
}

// Cleaner solution
// TODO: Learn more about rust iterators
pub fn merge2(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by_key(|val| val[0] );
    let mut res = Vec::with_capacity(intervals.len());
    let last = intervals.iter().fold((intervals[0][0], intervals[0][1]),|(l,r), curent| {
        if curent[0] > r {
            res.push(vec![l,r]);
            return (curent[0], curent[1])
        }
        (l, r.max(curent[1]))
    });
    res.push(vec![last.0, last.1]);
    res
}

fn main() {
    // let intervals = vec![vec![1,3],vec![2,6], vec![5, 7], vec![8,10],vec![15,18]];
    
    // let intervals = vec![vec![1,4], vec![0,4]];
    // let intervals = vec![vec![1,4], vec![0,0]];
    // let intervals = vec![vec![1,4], vec![0,1]];
    let intervals = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]];
    println!("{:?}", merge(intervals));
}
