pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
fn merge_two_lists(mut list1: Option<Box<ListNode>>,
                   mut list2: Option<Box<ListNode>>)
    -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    while list1.is_some() && list2.is_some() {
        let next_node;
        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val  {
            next_node = list1.take();
            list1=tail.next.take();
        }else {
            next_node = list2.take();
            list2=tail.next.take();

        }
    tail.next=next_node;
    tail=tail.next.as_mut().unwrap();
    }

    tail.next=list1.or(list2);
    dummy.next
}
fn print_list(mut list: Option<Box<ListNode>>) {
    while let Some(node) = list {
        print!("{} -> ", node.val);
        list = node.next;
    }
    println!("None");
}
fn main(){
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(5))),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(6))),
        })),
    }));
    let merged = merge_two_lists(list1, list2);
    print_list(merged);
}