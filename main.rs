mod input;
mod matrix;

fn main() {
    //input size for matrix
    let size = input::int();

    let mut null_mt = matrix::get_null_matrix(size as usize);

    matrix::print(&mut null_mt);

    //input matrix
    let mut matrix = input::matrix(size);

    matrix::print(&mut matrix);

    matrix::add_column(&mut matrix, (3, -1.), (1, 1.), 1);

    matrix::print(&mut matrix);

    let vec = matrix::get_column(matrix, 1);

    println!("{:?}", vec);

    

}