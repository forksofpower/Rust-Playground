pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    // pub fn length(&self) -> usize {
    //     self.queue.len()
    // }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    // pub fn peek(&self) -> Option<&T> {
    //     self.queue.first()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // queue 
}
mod queue;
fn main() {
    let mut q: queue::Queue<i32> = queue::Queue::new();
    q.enqueue(1);
    // assert_eq!(q.peek().unwrap(), 54345345);
    let item = q.dequeue();
    assert_eq!(item, 1);
    assert_eq!(q.is_empty(), true);
}
