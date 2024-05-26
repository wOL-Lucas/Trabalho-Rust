use std::collections::VecDeque;

pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}