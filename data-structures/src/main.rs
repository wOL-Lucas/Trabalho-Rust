use data_structures::stack::Stack;
use data_structures::list::List;

fn main() {
    println!("Hello, world!");

    test_stack();
    test_list();
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

fn test_list() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("List find {:?}", list.find(&2));
    println!("List remove {:?}", list.remove(&1));
    println!("List remove by index {:?}", list.remove_index(1));
}
