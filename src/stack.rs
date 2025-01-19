#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) {
        self.head.take().map(|node: Box<Node<T>>| {
            self.head = node.next;
            self.size -= 1;
            return node.value;
        });
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
