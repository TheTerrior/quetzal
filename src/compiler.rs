use std::{io::{BufReader, BufRead, Write, BufWriter}, fs::{File, self}, collections::HashMap};

use crate::error::QuetzalError;


///// Compiles Quetzal assembly into Quetzal bytecode
//pub fn compile(input: &str, output: &str) -> Result<(), QuetzalError> {
//    Ok(())
//}


pub struct BitVector {
    pub leftover: u8, //how many bits have been used up in the last u8 of the vector
    pub bytes: Vec<u8>,
}
impl BitVector {
    pub fn new() -> Self {
        BitVector { leftover: 0, bytes: Vec::new() }
    }

    pub fn with_capacity(size: usize) -> Self {
        BitVector { leftover: 0, bytes: Vec::with_capacity(size / 8) }
    }

    /// Push one byte onto the BitVector
    pub fn push(&mut self, byte: u8) {
        if self.leftover == 0 {
            self.bytes.push(byte);
            return;
        }
        *self.bytes.last_mut().unwrap() = self.bytes.last().unwrap() | (byte >> self.leftover); //push to the last byte
        self.bytes.push(0);
        //let operand = match self.leftover {
        //    1 => 1,
        //    2 => 3,
        //    3 => 7,
        //    4 => 15,
        //    5 => 31,
        //    6 => 63,
        //    7 => 127,
        //    _ => unreachable!()
        //};
        let operand = u8::pow(2, self.leftover as u32) - 1;
        *self.bytes.last_mut().unwrap() = self.bytes.last().unwrap() | ((byte & operand) << (8 - self.leftover)); //push to the new last byte
    }

    /// Push a certain number of bits onto the BitVector
    pub fn push_bytes(&mut self, bytes: &[u8], mut count: usize) -> Result<(), QuetzalError> {
        if bytes.len() * 8 < count {
            return Err(QuetzalError::NotEnoughBits); //ensures we don't try to push too many bits
        }
        for i in 0..bytes.len() {
            if count > 7 {
                self.push(bytes[i]); //usual case simply push to the BitVector
                count -= 8;
                if count == 0 {
                    return Ok(()); //finished pushing bits
                }
                continue;
            }
            // if reached here, we're pushing a specific number of bits
            return Ok(());
        }
        Ok(()) //should only be reachable if both the size of the bytes array and the count are 0
    }
}


///// Performs lexical analysis of the input data
/// Compiles Quetzal assembly into Quetzal bytecode line by line
pub fn compile(input: &str, output: &str) -> Result<(), QuetzalError> {
    let assembly_file = File::open(input).map_err(|_| QuetzalError::InputFileNotFound)?;
    let file_size = assembly_file.metadata().map_err(|_| QuetzalError::BadFileRead(String::from(input)))?.len(); //get the file's size
    let bytecode_file = fs::File::create(output).map_err(|_| QuetzalError::InputFileNotFound)?;
    let reader = BufReader::new(assembly_file); //don't move entire file into memory
    let writer = BufWriter::new(bytecode_file); //reduce number of writes to storage
    let mut variable_counter = 0;
    let mut variable_names: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let tokens: Vec<&str> = line.map_err(|_| QuetzalError::BadFileRead(String::from(input)))?.split(' ').collect(); //split line into tokens
        let bits = a();
        
    }
    Ok(())
}


#[inline]
pub fn a() -> Result<(), QuetzalError> {

    Ok(())
}

