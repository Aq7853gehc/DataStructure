pub struct LinkNode {
    val: Option<i32>,
    next: Option<Box<LinkNode>>,
}

impl LinkNode { 
    pub fn new_head() -> Self {
        LinkNode {
            val: None,
            next: None,
        }
    }

    fn new(data: i32) -> Self {
        LinkNode {
            val: Some(data),
            next: None,
        }
    }

    pub fn push(&mut self, data: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(LinkNode::new(data)))
    }

    pub fn print_list(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            println!("{:?}", node.val.unwrap_or(0));
            current = node.next.as_deref();
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.next.is_none() {
            self.val.take()
        } else {
            let mut current = self;
            while current.next.as_ref().unwrap().next.is_some() {
                current = current.next.as_mut().unwrap()
            }
            current.next.take().unwrap().val
        }
    }
}
