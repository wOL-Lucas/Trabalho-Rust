use data_structures::stack::Stack;
use data_structures::list::List;
use data_structures::queue::Queue;

fn main() {
    println!("Hello, world!");

    test_stack();
    test_list();
    test_queue(); 
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

fn test_queue() {
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("Front of the queue: {:?}", queue.peek());
    println!("Queue length: {}", queue.len());

    while let Some(element) = queue.dequeue() {
        println!("Dequeued: {}", element);
    }

    println!("Queue is empty: {}", queue.is_empty());
}
