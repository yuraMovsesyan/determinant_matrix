pub fn print(matrix: &mut Vec<Vec<f64>>) {
    println!("");
    let mut f = 1;
    for i in matrix{
        if f == 0 {
            f += 1;
            continue;
        }
        println!("{:?}", i);
    }
    println!("");
}

pub fn swap_row(matrix: &mut Vec<Vec<f64>>, a: usize, b: usize) {
    for i in 0..matrix[0][2] as usize{
        let c = matrix[a][i];
        matrix[a][i] = matrix[b][i];
        matrix[b][i] = c;
    }

    matrix[0][0] = if matrix[0][0] == 0. { 1. } else { 0. };
}

pub fn swap_column(matrix: &mut Vec<Vec<f64>>, a: usize, b: usize) {
    let a: usize = a - 1;
    let b: usize = b - 1;
    for i in 1..(matrix[0][1] + 1.) as usize{
        let c = matrix[i][a];
        matrix[i][a] = matrix[i][b];
        matrix[i][b] = c;
    }

    matrix[0][0] = if matrix[0][0] == 0. { 1. } else { 0. };
}

pub fn add_row(matrix: &mut Vec<Vec<f64>>, a: (usize, f64), b: (usize, f64), s: usize) {
    for i in 0..matrix[0][2] as usize{
        let c = matrix[a.0][i] * a.1 + matrix[b.0][i] * b.1;
        matrix[s][i] = c;
    }
}

pub fn add_column(matrix: &mut Vec<Vec<f64>>, a: (usize, f64), b: (usize, f64), s: usize) {
    let a: (usize, f64) = (a.0 - 1, a.1);
    let b: (usize, f64) = (b.0 - 1, b.1);
    let s: usize = s - 1;
    for i in 1..(matrix[0][1] + 1.) as usize{
        let c = matrix[i][a.0] * a.1 + matrix[i][b.0] * b.1;
        matrix[i][s] = c;
    }
}

pub fn get_column(matrix: &mut Vec<Vec<f64>>, a: usize) -> Vec<f64> {
    let mut vec_column:Vec<f64> = Vec::new();

    for i in 1..(matrix[0][1] + 1.) as usize{
        vec_column.push(matrix[i][a]);
    }

    vec_column
}

pub fn set_column(matrix: Vec<Vec<f64>>, val_vec: Vec<f64>, index: usize) -> Vec<Vec<f64>>{

    let mut matrix = matrix;

    let height = matrix[0][1] as usize;

    for h in 1..(height + 1){
        matrix[h][index] = val_vec[h - 1];
    }

    matrix
}

pub fn remove_column(matrix: Vec<Vec<f64>>, index: usize) -> Vec<Vec<f64>>{
    let mut matrix = matrix;
    let height = matrix[0][1] as usize;
    matrix[0][2] = matrix[0][2] - 1.;
    for id in 1..(height + 1){
        matrix[id].remove(index);
    }
    matrix
}
pub fn get_null_matrix(size: usize) -> Vec<Vec<f64>> {
    let mut null_matrix: Vec<Vec<f64>>= Vec::new();
    let info:Vec<f64> = vec![0., size as f64, size as f64];
    null_matrix.push(info);

    for i in 0..size{
        let mut vec: Vec<f64>= vec![0.; size];
        vec[i] = 1.;
        null_matrix.push(vec);
    }

    null_matrix
}

pub fn get_index_min_down(vec: &Vec<f64>, start: usize) -> usize {
    let len = vec.len();
    let mut min = vec[start];
    let mut min_id = start;

    for i in start..len {
        if vec[i] < min && vec[i] != 0.{
            min = vec[i];
            min_id = i;
        }
    }

    min_id
}

pub fn det(matrix: Vec<Vec<f64>>) -> f64 {
    let size = matrix[0][1];
    let mut matrix = matrix;

    let mut result = 1.;

    for i in 0..size as usize {
        let vec = get_column(&mut matrix, i);

        let min_id = get_index_min_down(&vec, i);

        if matrix[min_id + 1][i] != matrix[i + 1][i]{
            swap_row(&mut matrix, min_id + 1, i + 1);
            //println!("swap {} {}", min_id + 1, i + 1);
        }

        for j in (i + 1)..(size) as usize{
            let k = (0. - matrix[j + 1][i]) / matrix[i + 1][i];
            add_row(&mut matrix, (i + 1, k), (j + 1, 1.), j + 1);
        }

        //print!("{:?} {}\n ", matrix[i + 1][i], min_id);

        result *= matrix[i + 1][i];
    }

    //print(&mut matrix);

    let k = if matrix[0][0] == 0. { 1. } else { -1. };
    result *= k;

    let res = result;
    if res.floor() + 0.5 < result{
        return res.ceil();
    }
    else{
        return res.floor();
    }
}

pub fn inv(matrix: Vec<Vec<f64>>){
    let size = matrix[0][1];
    let mut matrix = matrix;

    let mut null_matrix = get_null_matrix(size as usize);

    let mut result = 1.;

    for i in 0..size as usize {
        let vec = get_column(&mut matrix, i);

        let min_id = get_index_min_down(&vec, i);
        /*
        if matrix[min_id + 1][i] != matrix[i + 1][i]{
            swap_row(&mut matrix, min_id + 1, i + 1);
            swap_row(&mut null_matrix, min_id + 1, i + 1);
            println!("swap {} {}", min_id + 1, i + 1);
        }
        */
        if i == 0{
            let id_end = size as usize;
            for j in 2..size as usize{
                let k = (0. - matrix[j][id_end - 1]) / matrix[1][id_end - 1];
                add_row(&mut matrix, (1, k), (j, 1.), j);

                add_row(&mut null_matrix, (1, k), (j, 1.), j);
            }

            let k = (1. - matrix[id_end][id_end - 1]) / matrix[1][id_end - 1];
            add_row(&mut matrix, (id_end, 1.), (1, k), id_end);

            add_row(&mut null_matrix, (id_end, 1.), (1, k), id_end);

            let k = (0. - matrix[1][id_end - 1]) / matrix[id_end][id_end - 1];
            add_row(&mut matrix, (1, 1.), (id_end, k), 1);

            add_row(&mut null_matrix, (1, 1.), (id_end, k), 1);

            println!("----------------------");
            print(&mut matrix);
            println!("----------------------");
        }

        if matrix[i + 1][i] != 1.{
            for j in (i + 1)..(size) as usize{
                if matrix[j + 1][i] != 0.{
                    let k = (1. - matrix[i + 1][i]) / matrix[j + 1][i];
                    add_row(&mut matrix, (i + 1, 1.), (j + 1, k), i + 1);

                    add_row(&mut null_matrix, (i + 1, 1.), (j + 1, k), i + 1);

                    
                    println!("-----3333333333-----------------");
                    print(&mut matrix);
                    println!("----------33333333333------------");
                    break;
                }
            }
        }

        if i + 1 == size as usize{
            let k = (1. - matrix[i + 1][i]) / matrix[i + 1][i];
            add_column(&mut matrix, (i + 1, 1.), (i + 1, k), i + 1);

            add_column(&mut null_matrix, (i + 1, 1.), (i + 1, k), i + 1);

            
            println!("-----3333333333-----------------");
            print(&mut matrix);
            println!("----------33333333333------------");
        }

        
        for j in (i + 1)..(size) as usize{
            let k = (0. - matrix[j + 1][i]) / matrix[i + 1][i];
            add_row(&mut matrix, (i + 1, k), (j + 1, 1.), j + 1);

            add_row(&mut null_matrix, (i + 1, k), (j + 1, 1.), j + 1);
        }

        print!("{:?} {}\n ", matrix[i + 1][i], min_id);
    }

    print(&mut matrix);
    print(&mut null_matrix);

    let size = matrix[0][2];

    for i in 0..size as usize {
        for j in 0..i as usize{
            let k = (0. - matrix[j + 1][i]) / matrix[i + 1][i];
            add_row(&mut matrix, (i + 1, k), (j + 1, 1.), j + 1);

            add_row(&mut null_matrix, (i + 1, k), (j + 1, 1.), j + 1);
        }
    }

    print(&mut matrix);
    print(&mut null_matrix);
}

pub fn step(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let height = matrix[0][1] as usize;
    let width=matrix[0][2] as usize;

    let mut matrix = matrix;
    let mut w = 0 as usize;

    for h in 1..(height + 1) {
        while matrix[h][w] == 0. {
            let vec = get_column(&mut matrix, w);

            println!("vec {:?}, {}, {}", &vec[h..height], vec.len(), height - h);
            
            if height - h == 0 { break; }
            for i in (h + 1)..(height + 1){
                println!("!!!!--- {}, i = {}", matrix[i][w], i);
                if matrix[i][w] != 0.{
                    println!("--- {}", matrix[i][w]);
                    swap_row(&mut matrix, i, h);
                    print(&mut matrix);
                    break;
                }
            }

            if width - w > 1 && matrix[h][w] == 0.{
                w += 1;
            }

        }


        println!("{}", matrix[h][w]);

        for hh in (h + 1)..(height + 1){
            let k = (0. - matrix[hh][w]) / matrix[h][w];
            add_row(&mut matrix, (h, k), (hh, 1.), hh);
        }


        if width - w == 1 { break; }
        else{ w += 1;}
    }

    matrix
}

pub fn rank(matrix: Vec<Vec<f64>>) -> i32 {
    let mut matrix = step(matrix.clone());
    let matrix_len = matrix[1].len();

    print(&mut matrix);

    let mut sum = 0;
    for row in &matrix[1..]{
        let mut flag = false;
        for column in row{
            if column > &0.00000000000001{
                flag = true;
                break;
            }
        }
        if flag {
            sum += 1;
        }else{
            break;
        }
    }

    sum
}

pub fn kramer(matrix: Vec<Vec<f64>>) -> Vec<f64>{
    let mut matrix = matrix;
    let width = matrix[0][2] as usize;
    let vec_ratio = get_column(&mut matrix, width - 1);


    let mut result:Vec<f64> = Vec::new();

    let mut matrix = remove_column(matrix.clone(), width - 1);
    let matrix_det = det(matrix.clone());

    let width = matrix[0][2] as usize;

    for w in 0..width {

        let delta = set_column(matrix.clone(), vec_ratio.clone(), w);

        let delta_det = det(delta.clone());

        let res = delta_det / matrix_det;

        result.push(res);
    }

    result
}
