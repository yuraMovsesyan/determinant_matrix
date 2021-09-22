extern crate rand;

use rand::Rng;

fn main()
{
    const MATRIX_SIZE: usize = 4;
    let mut isEven = true;
    let mut rng = rand::thread_rng();

    let mut numbers:Vec<Vec<f64>> = vec![vec![0.0; MATRIX_SIZE]; MATRIX_SIZE];

    for y in 0..MATRIX_SIZE{
        for x in 0..MATRIX_SIZE{
            numbers[y][x] = (rng.gen::<u32>() % 10) as f64;
            print!("{} ",  numbers[y][x]);
        }
        println!("");
    }

    for y in 0..MATRIX_SIZE{
        if (numbers[0][0] < numbers[y][0]){
            numbers.swap(0, y);
            isEven = !isEven;
        }
    }

    for index in 0..(MATRIX_SIZE - 1){
        for y in (index + 1)..MATRIX_SIZE{
            let ratio = (0.0 - numbers[y][index]) / numbers[index][index];
            for x in 0..MATRIX_SIZE{
                numbers[y][x] = numbers[y][x] + numbers[index][x] * ratio;
            }
        }
    }
    let mut result:f64 = 1.0;
    for y in 0..MATRIX_SIZE{
        result *= numbers[y][y];
    }
    if (!isEven){
        result *= (-1.0);
    }
    let res = result;
    if (res.floor() + 0.5 < result){
        println!("\nDeterminant: {}", res.ceil());
    }
    else{

        println!("\nDeterminant: {}", res.floor());
    }
}