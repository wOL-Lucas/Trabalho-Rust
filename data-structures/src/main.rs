use data_structures::stack::Stack;

fn main() {
    println!("Hello, world!");

    test_stack();
}


fn test_stack() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack peek {:?}", stack.peek());
    println!("Stack pop {:?}", stack.pop());
    println!("Stack pop {:?}", stack.pop());
}
