use std::ops::DerefMut;

pub struct LinkNode {
    val: i32,
    next: Option<Box<LinkNode>>,
}

impl LinkNode {
    pub fn new(data: i32) -> Self {
        LinkNode {
            val: data,
            next: None,
        }
    }

    pub fn add_node(&mut self, data: i32) {
        let mut current: &mut LinkNode = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(LinkNode::new(data)))
    }

    pub fn print_list(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            println!("{:?}", node.val);
            current = node.next.as_deref();
        }
    }

    // pub fn insert_at_beg(& mut self, data:i32){
    //     let  current = self;
    //     let  new_node = Box::new(LinkNode::new(data));
         
    // }
}
