//! 工具包
//!
use common::ListNode;

include!("functions/lib.rs");

/// ### 打印链表
pub fn print_list_node(list: &Option<Box<ListNode>>) {
    match list {
        None => println!(),
        Some(list) => {
            print!("{}\t", list.val);
            print_list_node(&list.next);
        }
    }
}

/// ### List Node To String
pub fn list_node_to_str(list: &Option<Box<ListNode>>) -> String {
    match list {
        None => String::from("\n"),
        Some(list) => String::from(list.val.to_string() + "\t") + &list_node_to_str(&list.next),
    }
}

#[cfg(test)]
mod tests {
    use crate::{array_to_list_node, list_node_to_str, print_list_node};

    #[test]
    fn test_array_to_list_node() {
        let list = array_to_list_node(Box::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 6, 7, 3, 3]));
        print_list_node(&list);
    }

    #[test]
    fn test_list_node_to_str() {
        let list = array_to_list_node(Box::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 6, 7, 3, 3]));
        let list_str = list_node_to_str(&list);
        println!("{}", list_str);
    }
}
