use core::fmt;

/// Errors for Quetzal.
#[derive(Debug)]
pub enum QuetzalError {
    EmptyStack, 
    ExcessiveFrameSize, 
    ExcessiveStackSize, 
    InputFileNotFound,
    OutputFileNotCreated,
    BadFileRead(String),
    NotEnoughBits,
}
impl std::error::Error for QuetzalError {}
impl fmt::Display for QuetzalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyStack => write!(f, "Tried to pop a stack frame but the stack was empty."),
            Self::ExcessiveFrameSize => write!(f, "Tried to define a variable but the frame was too large."),
            Self::ExcessiveStackSize => write!(f, "Tried to push a new frame onto the stack but the stack was too large, stack overflow."),
            Self::InputFileNotFound => write!(f, "Couldn't find the provided file."),
            Self::OutputFileNotCreated => write!(f, "Couldn't create the specified file."),
            Self::BadFileRead(file) => write!(f, "Ran into a problem reading the file '{}'.", file),
            Self::NotEnoughBits => write!(f, "Couldn't push the number of bits specified."),
        }
    }
}
