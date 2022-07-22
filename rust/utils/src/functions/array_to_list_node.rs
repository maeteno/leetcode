/// ### 数组转链表
pub fn array_to_list_node(vec: Box<&[i32]>) -> Option<Box<ListNode>> {
    let node = vec.get(0);

    match node {
        None => None,
        Some(val) => {
            let len = vec.len();
            let mut next_node: Option<Box<ListNode>> = None;
            if len > 1 {
                let left = &vec[1..len];
                next_node = array_to_list_node(Box::new(left));
            }

            Some(Box::new(ListNode {
                val: *val,
                next: next_node,
            }))
        }
    }
}