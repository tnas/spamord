use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use sprs::{CsMatI, TriMatBase};

fn parse_arguments() -> (String, usize) {
    
    let args: Vec<String> = env::args().collect();
    let num_threads: usize;
    let matrix_file: String;

    match args.len() {
        3 => {
            matrix_file = String::from(&args[1]);
            num_threads = match args[2].parse::<usize>() {
                Ok(n) => n,
                Err(_) => panic!("error: second argument not an integer")
            };
        }
        _ => panic!("Pass -g for graph builder or -d for dynamic neighborhood and the number of threads!")
        
    }

    return (matrix_file, num_threads);
}



fn load_symmetric_csr_matrix(path_file: &String) -> CsMatI<f32, usize> {

    let file = match File::open(path_file) {
        Ok(f) => f,
        Err(_) => panic!("Failure by opening the file {}", path_file)
    };

    let mut reader = BufReader::new(file).lines();
    let mut tokens;

    let mut header; // Reading the matrix's dimensions
    loop {
        header = reader.next().unwrap().expect("Failure reading the matrix's header");
        if ! header.starts_with("%") { break; }
    }

    tokens = header.split_whitespace();
    let n_rows: usize = tokens.next().unwrap().parse().expect("Failure parsing number of rows of matrix");
    let n_cols: usize = tokens.next().unwrap().parse().expect("Failure parsing number of columns of matrix");
    let _nnz: u32     = tokens.next().unwrap().parse().expect("Failure parsing number of non-zeros of matrix");

    let mut mat_base: TriMatBase<Vec<usize>, Vec<f32>> = TriMatBase::new((n_rows, n_cols));

    let mut row: usize;
    let mut col: usize;
    let mut val: f32;

    for line in reader.map(|l| l.unwrap()) {
        tokens = line.split_whitespace();
        row = tokens.next().unwrap().parse().expect("Failure parsing row value");
        col = tokens.next().unwrap().parse().expect("Failure parsing column value");
        val = tokens.next().unwrap().parse().expect("Failure parsing position value");
        mat_base.add_triplet(row - 1, col - 1, val);
    }

    let csr_mat: CsMatI<f32, usize> = mat_base.to_csr();

    csr_mat
}



fn main() {
    let (file, _nthreads) = parse_arguments();
    let csr_mat = load_symmetric_csr_matrix(&file);
    println!("nnz: {}", csr_mat.nnz());
}
