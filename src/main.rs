mod linked_list;
use linked_list::LinkNode;

fn main() {
    let mut n1 = LinkNode::new(0);
    n1.add_node(20);
    n1.add_node(32);
    n1.add_node(34);
    n1.print_list();
}
