mod memory;
mod alu;
mod error;
mod compiler;
#[cfg(test)]
mod tests;


fn main() {
    let mut bits: Vec<u8> = Vec::new();
    let x: usize = 10064;
    let y = x.to_be_bytes();
    let z = 0x1;
    let a: u8 = 255;
    println!("{:?}", y);
    println!("{:?}", z);

    bits.push(0);
    bits[0] = bits[0] | (z << 7);
    let offset = 1;

    bits[0] = bits[0] | (a as u8 >> 1); //grab all but the last 1, palce at end of byte
    bits.push(0);

    bits[1] = bits[1] | ((a as u8 & 1) << 7); //grab only the last 1, place at start of byte




    println!("{:?}", bits);
    println!("Hello, world!");
}

