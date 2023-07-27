
/// Proportion of all elements to search for (n/K)
const K: usize = 3;

/// Implement a reducible bag
#[derive(Debug, Default)]
struct Bag {
    bag: Vec<(i32, i32)>,
}

impl Bag {
    pub fn new() -> Bag {
        Default::default()
    }

    pub fn add(&mut self, element: i32) {
        for (value, count) in &mut self.bag {
            if *value == element {
                *count += 1;
                return;
            }
        }
        self.bag.push((element, 1));
    }

    /// Performs bag-reduction through count decrement.
    /// Popular candidates will remain in the bag over the long term
    pub fn reduce(&mut self) {
        for (_, ref mut c) in &mut self.bag {
            *c -= 1;
        }

        // Retain only nonzero-count elements
        self.bag.retain(|(_, c)| *c > 0);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bag.len()
    }

    pub fn get(&self) -> &Vec<(i32, i32)> {
        &self.bag
    }
}

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut bag = Bag::new();
    
    // Find candidate hitters through bag-reduction
    for num in &nums {
        bag.add(*num);

        if bag.len() >= K {
            bag.reduce();
        }
    }

    bag.get()
        .iter()
        .filter_map(|&(e, _) | {
            if nums.iter().filter(|&&g| e == g).count() > nums.len() / K {
                Some(e)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let nums = vec![2,1,1,3,1,4,5,6];
    // let nums = vec![3, 2, 3];
    // let nums = vec![1];
    // let nums = vec![1,2];

    let result = majority_element(nums);
    println!("result = {:?}", result);
}
