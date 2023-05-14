pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let size = matrix.len();
    let mut temp;

    // Transpose
    for row in 0..size {
        for col in 0..row {
            temp = matrix[row][col];
            matrix[row][col] = matrix[col][row];
            matrix[col][row] = temp;
        }
    }

    // Reverse
    for i in 0..size {
        matrix[i].reverse();
    }
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

fn main() {
    // let mut matrix = vec! 
    //     [
    //         vec![1,2,3],
    //         vec![4,5,6],
    //         vec![7,8,9]
    //     ];
    let mut matrix = vec!
        [
            vec![5,1,9,11],
            vec![2,4,8,10],
            vec![13,3,6,7],
            vec![15,14,12,16]
        ];

    rotate(&mut matrix);
    print_matrix(&matrix);
}
