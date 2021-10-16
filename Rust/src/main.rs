mod input;
mod matrix;

fn main() {
    //input size for matrix
    let size = input::int();

    let mut matrix = input::matrix(size);

    let mut rank = matrix::step(matrix.clone());
    matrix::print(&mut matrix);

    matrix::print(&mut rank);

    let le = matrix[1].len();

    let v:Vec<f64> = matrix::get_column(&mut matrix, le - 1);
    


    println!("line: {:?}",v)

    //matrix::inv(matrix.clone());

}