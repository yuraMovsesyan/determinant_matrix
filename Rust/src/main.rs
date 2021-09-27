mod input;
mod matrix;

fn main() {
    //input size for matrix
    let size = input::int();

    let mut matrix = input::matrix(size);

    matrix::print(&mut matrix);

    for i in 0..size as usize {
        let vec = matrix::get_column(&mut matrix, i + 1);

        let min_id = matrix::get_index_min(&vec, i);

        for j in i..size as usize{
            let k = (0.0 - matrix[j][i]) / matrix[i + 1][i];
            matrix::add_row(&mut matrix, (i, k), (j, 1.), j)
        }

        print!("{:?} {}", matrix[i + 1][i], min_id);
    }

    matrix::print(&mut matrix);

}