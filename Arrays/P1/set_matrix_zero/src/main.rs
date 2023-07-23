fn print_matrix(matrix: Vec<Vec<i32>>) {
    for row in matrix {
        for elem in row {
            print!("{} ", elem);
        }
        println!("")
    }
}

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut col0 = 1;
    
    for y in 0..rows {
        if matrix[y][0] == 0 { col0 = 0 };
        for x in 1..cols {
            if matrix[y][x] == 0 {
                matrix[0][x] = 0;
                matrix[y][0] = 0;
            }
        }
    }

    for y in (0..rows).rev() {
        for x in (1..cols).rev() {
            if matrix[0][x] == 0 || matrix[y][0] == 0 {
                matrix[y][x] = 0;
            }            
        }
        if col0 == 0 {
            matrix [y][0] = 0;
        }
    }
}

fn main() {
    let mut matrix = vec![
        vec![1, 1, 2, 0],
        vec![0, 2, 5, 2],
        vec![1, 3, 4, 3]
    ];

    set_zeroes(&mut matrix);

    print_matrix(matrix);
}