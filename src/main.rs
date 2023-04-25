#[derive(Debug)]
struct Queue{
    val:i32,
    next: Option<Box<Queue>>
}

impl Queue{
    fn new(val:i32)->Queue{
        Queue { val, next:None}
    }
    fn enqueue(&mut self, val:i32){
        if let Some(ref mut next)=self.next{
            next.enqueue(val);
        } else {
            self.next = Some(Box::new(Queue::new(val)));
        }
    }
    fn deque(&mut self){
        if let Some(next)=self.next{
            self.
        }
    }
}

fn main() {
    let mut queue = Queue::new(3);
    println!("{queue:?}");
    queue.enqueue(5);
    println!("{queue:?}");
}