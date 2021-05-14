use std::fs::File;
use std::io::{self, prelude::*, BufReader};



pub struct Matrix<T> {
    internal_rep_: Vec<T>,
    pub dim: usize,
}
impl<T> Matrix<T> 
where
    T: Clone+std::str::FromStr+Copy {

    pub fn new_with(dim: usize, fill: T) -> Matrix<T> {
        Matrix {
            internal_rep_: vec![fill; dim*dim],
            dim,
        }
    }

    pub fn new_from(fname: &str, default_val: T) -> Result<Matrix<T>, io::Error> {

        let file = File::open(fname)?;
        let mut reader = BufReader::new(file);

        //get cached matrix dim from first line
        let mut dim_str = String::new();
        reader.read_line(&mut dim_str)?;
        let dim = dim_str.trim().parse::<usize>().unwrap();
        println!("len: {}", dim);

        let mut mat = Matrix::new_with(dim, default_val);

        for (row_i, line) in reader.lines().enumerate() {
            let row: Vec<T> = (line?.split_whitespace()
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|x| match x.parse::<T>() {
                                    Ok(parsed) => parsed,
                                    Err(_) => panic!("error parsing matrix!"),
                                })).collect::<Vec<T>>();
            for col_i in 0..row.len() {
                *(mat).at(row_i, col_i) = row[col_i]; 
            }
        }

        Ok(mat)
    }

    pub fn at(&mut self, row: usize, col: usize) -> &mut T {
        let dim = self.dim;
        &mut self.internal_rep_[row*dim + col]
    }

    pub fn at_read(&self, row: usize, col: usize) -> &T {
        let dim = self.dim;
        &self.internal_rep_[row*dim + col]
    }
}

impl<T> Matrix<T>
where
    T: Clone+std::fmt::Display+std::str::FromStr+Copy {
    pub fn write_to(mat: &Matrix<T>, fname: &str) -> Result<(), io::Error> {
        
        let mut buffer = File::create(fname)?; 
        buffer.write_all(format!("{}\n", mat.dim).as_bytes())?;
        
        for row_i in 0..mat.dim {
            for col_i in 0..mat.dim {
                buffer.write_all(format!("{}\t", mat.at_read(row_i, col_i)).as_bytes())?;
            }
            buffer.write_all(b"\n")?;
        }

        Ok(())
    }
}