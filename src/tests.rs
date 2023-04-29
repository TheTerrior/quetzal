use crate::memory;


/// Push a variable to a stack, retrieve said variable
#[test]
fn variable_memory() {
    let mut mem = memory::Stack::new();
    mem.push();
    let var = memory::Variable::Bool(true);
    mem.define(1, var);
    let result = mem.search(1);
    assert_eq!(result, Some(memory::Variable::Bool(true)));
}


/// Retrieve an undefined variable
#[test]
fn variable_memory1() {
    let mut mem = memory::Stack::new();
    mem.push();
    let result = mem.search(1);
    assert_eq!(result, None);
}





