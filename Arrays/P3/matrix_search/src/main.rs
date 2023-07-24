pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
	let (mut start, mut mid, mut end) = (0, 0, matrix.len() - 1);
	
	while start <= end {
		mid = start + (end - start) / 2;

		if matrix[mid][0] == target {
			return true;
		} else if matrix[mid][0] < target {
			if *matrix[mid].last().unwrap() >= target{
				return binary_search(&matrix[mid], target).is_some();
			}
			start = mid + 1;
		} else {
			if mid == 0 {
				return false;
			}
			end = mid - 1;
		}
	}
	false
}

pub fn binary_search(arr: &Vec<i32>, target: i32) -> Option<usize> {
	let (mut start, mut mid, mut end) = (0, 0, arr.len() - 1);
	while start <= end {
			mid = start + (end - start) / 2;

			if arr[mid] == target {
					return Some(mid);
			} else if arr[mid] < target {
					start = mid + 1;
			} else {
					if mid == 0 {
							return None;
					}
					end = mid - 1;
			}
	}

	None
}


fn main() {
	let matrix = 
	vec![
		vec![1,3,5,7],
		vec![10,11,16,20],
		vec![23,30,34,60]
	];
	let matrix = vec![vec![1]];
	let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,50]];
	let target = 11;

	match search_matrix(matrix, target) {
		true => println!("Found"),
		false => println!("Not found")
	}
}
