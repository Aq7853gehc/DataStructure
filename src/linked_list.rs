pub struct LinkNode {
    val: Option<i32>,
    next: Option<Box<LinkNode>>,
}
impl LinkNode     {
    pub fn new_head()->Self{
        LinkNode{
            val:None,
            next:None}
    }
    pub fn new(data:i32) -> Self {
        LinkNode {
            val: Some(data),
            next: None,
        }
    }

    pub fn add_node(&mut self, data: i32 ) {
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

    pub fn add_beg(&mut self, data:i32){
        let mut node_b = LinkNode::new(data);
        node_b.next = self.next.take();
        self.next  = Some(Box::new(node_b));
    }

   
}
