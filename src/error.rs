use core::fmt;

/// Errors for Quetzal.
#[derive(Debug)]
pub enum QuetzalError {
    EmptyStack, 
    ExcessiveFrameSize, 
    ExcessiveStackSize, 
}
impl std::error::Error for QuetzalError {}
impl fmt::Display for QuetzalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyStack => write!(f, "Tried to pop a stack frame but the stack was empty."),
            Self::ExcessiveFrameSize => write!(f, "Tried to define a variable but the frame was too large."),
            Self::ExcessiveStackSize => write!(f, "Tried to push a new frame onto the stack but the stack was too large, stack overflow."),
        }
    }
}
