mod input;
mod matrix;

fn main() {
    //input size for matrix
    let size = input::int();

    let mut matrix = input::matrix(size);

    let det = matrix::det(matrix.clone());
    matrix::print(&mut matrix);

    println!("Det: {}", det);

}