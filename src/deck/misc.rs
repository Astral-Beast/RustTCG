use std::fmt;

pub fn print_vec<T>(vec: &Vec<T>)
where
    T: fmt::Display, //requires Display to be implemented for the type T in the vector
{
    for att in vec {
        println!("{}", att);
    }
}