use std::{io::{BufReader, BufRead, Write, BufWriter}, fs::{File, self}, collections::HashMap};

use crate::error::QuetzalError;


///// Compiles Quetzal assembly into Quetzal bytecode
//pub fn compile(input: &str, output: &str) -> Result<(), QuetzalError> {
//    Ok(())
//}


///// Performs lexical analysis of the input data
/// Compiles Quetzal assembly into Quetzal bytecode line by line
pub fn compile(input: &str, output: &str) -> Result<(), QuetzalError> {
    let assembly_file = File::open(input).map_err(|_| QuetzalError::InputFileNotFound)?;
    let bytecode_file = fs::File::create(output).map_err(|_| QuetzalError::InputFileNotFound)?;
    let reader = BufReader::new(assembly_file); //don't move entire file into memory
    let writer = BufWriter::new(bytecode_file); //reduce number of writes to storage
    let mut variable_counter = 0;
    let mut variable_names: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let tokens: Vec<&str> = line.map_err(|_| QuetzalError::BadFileRead(String::from(input)))?.split(' ').collect(); //split line into tokens
        
    }
    Ok(())
}


#[inline]
pub fn a() {

}

