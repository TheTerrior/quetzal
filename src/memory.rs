use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}, cell::RefCell, isize};

use crate::error::QuetzalError;


#[derive(Clone, Debug)]
pub struct Stack {
    frames: Vec<HashMap<usize, Variable>>,
    temp: Vec<Variable>, //holds temporary values, ie during computation of expressions
}
impl Stack {
    pub fn new() -> Self {
        Stack {frames: Vec::new(), temp: Vec::with_capacity(3)} 
    }

    /// Searches for the id starting from the top frame.
    pub fn search(&self, id: usize) -> Option<Variable> {
        for i in (0..self.frames.len()).rev() { //iterate through all stack frames
            if self.frames[i].contains_key(&id) {
                return Some(self.frames[i].get(&id).unwrap().clone()); //the clone copies simple types, refs complex types
            }
        }
        return None; 
    }

    /// Defines a variable in the top frame. TODO test if we can remove the first if statement
    ///
    /// Returns EmptyStack if the stack has no frames.
    ///// Returns ExcessiveFrameSize if the frame is too big.
    pub fn define(&mut self, id: usize, var: Variable) -> Result<(), QuetzalError> {
        if self.frames.len() == 0 {
            return Err(QuetzalError::EmptyStack);
        }
        let frame = self.frames.last_mut().ok_or(QuetzalError::EmptyStack)?;
        //if frame.len() == isize::MAX as usize {
        //    return Err(QuetzalError::ExcessiveFrameSize);
        //}
        frame.insert(id, var);
        Ok(())
    }

    /// Pushes a new frame onto the stack.
    ///
    /// Returns ExcessiveStackSize if the stack is too large.
    pub fn push(&mut self) -> Result<(), QuetzalError> {
        if self.frames.len() == isize::MAX as usize {
            return Err(QuetzalError::ExcessiveStackSize);
        }
        self.frames.push(HashMap::new());
        Ok(())
    }

    /// Pops a frame from the stack.
    ///
    /// Returns EmptyStack if the stack has no frames.
    pub fn pop(&mut self) -> Result<(), QuetzalError> {
        if self.frames.len() == 0 {
            Err(QuetzalError::EmptyStack)
        } else {
            self.frames.pop();
            Ok(())
        }
    }

}



//#[derive(Clone, Debug)]
//pub struct StackFrame {
//    variables: Vec<Variable>, //the variables in this scope
//}



#[derive(Clone, Debug)]
pub enum Variable {

    //simple types
    Bool(bool),
    Char(char),
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
}



