fn generate(num_rows: i32) -> Vec<Vec<i32>> {    
    let mut result = vec![];
    for i in 0..num_rows as usize {
        result.push(vec![1; i + 1]);
        for k in 1..i {
            result[i][k] = result[i - 1][k - 1] + result[i - 1][k];
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_rows = input.trim().parse().unwrap();
    
    generate(num_rows).into_iter().for_each(|row: Vec<i32>| {
        for elem in row {
            print!("{} ", elem);
        }
        println!("");
    });
}

