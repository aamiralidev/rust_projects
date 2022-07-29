use image::io::Reader as ImageReader;
use std::process::exit;


fn if_duplicate(arr: &[u8; 9]) -> bool {
    // it takes an array of 9 elements and if there is any duplicate than return true
    if (1..arr.len()).any(|i| arr[i..].contains(&arr[i-1])){
        return true
    }
    false
}

fn verify_intensity(img: &[[u8; 9]; 9]) {
    // this function checks for every item of 9x9 2d array
    // and if it's not between 1 to 9 than exit with 1
    for row in img {
        for item  in row {
            if *item > 9 || *item == 0 {
                // println!("Intensity verification failed");
                exit(1);
            }
        }
    }
}

fn verify_cols_and_rows(img: &[[u8; 9]; 9]){
    //this functions in each iteration takes one row and one col
    //it checks for duplicate and if any, exit with code 1
    let mut col = [0 as u8; 9];
    for i in 0..9 {
        for j in 0..9 {
            col[j]= img[i][j];
        }
        if if_duplicate(&img[i]) || if_duplicate(&col) {
            // println!("found duplicate at {}th row or col", i+1);
            exit(1);
        }
    }
}

fn verify_submaps(img: &[[u8; 9]; 9]) {
    // it takes a 2d 9x9 array, split to 9 3x3 arrays
    // and checks if there is any duplicate, if duplicate than exit with code 1
    // let mut index;
    let mut submap = [0 as u8; 9];
    for i in 0..9 {
        for row in 0..3 {
            for col in 0..3 {
                submap[col + row*3] = img[row + (i/3)*3][col + (3*i)%9];
            }
        }
        if if_duplicate(&submap) {
            // println!("found duplicate in {}th box", i+1);
            exit(1);
        }
    }
}

fn main(){
    // get the commandline argument if any otherwise none
    // although numbering starts from 0
    // since the 0th argument is the binary name so we are getting first argument
    let mut filename = std::env::args().nth(1);
    let file;
    //if file name is passed through commandline than just load the image
    //otherwise take argument through standard input channel

    if let Some(file_n) = filename {
        file = ImageReader::open(file_n).unwrap().decode().unwrap();
    }else {
        let mut file_name = String::new();
        println!("Enter file name :");
        std::io::stdin().read_line(&mut file_name).unwrap();
        filename = Option::from(file_name);
        file = ImageReader::open(filename.unwrap()
            // since input from commandline also includes newline char so we are striping it
            .strip_suffix("\n").unwrap()).unwrap().decode().unwrap();
    }
    // converting Dynamic image buffer to GrayImage
    let file_n = file.to_luma8();

    // println!("{:?}", file_n.dimensions());

    if file_n.dimensions() != (9, 9) {
        // println!("dimension verification failed (9, 9)");
        exit(1);
    }
    // converting image to 9x9 2d array
    let mut img = [[0 as u8; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            img[i][j] = file_n.get_pixel(i as u32, j as u32).0[0];
        }
    }
    // printing all values
    // for row in img.iter() {
    //     println!("{:?}", row);
    // }

    //verifying constraints
    verify_intensity(&img);
    verify_cols_and_rows(&img);
    verify_submaps(&img);

    // if all constraints verified successfully, than exit with 0
    exit(0);
}
