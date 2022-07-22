use common::ListNode;

/// #### \[02\]两数相加
///
/// > 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
/// 请你将两个数相加，并以相同形式返回一个表示和的链表。
/// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    carried(l1, l2, 0)
}

pub fn carried(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                carry / 10,
            ),
            val: carry % 10,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::no_02_easy::add_two_numbers;
    use utils::{array_to_list_node, print_list_node};

    #[test]
    fn test_add_two_numbers() {
        let l1 = array_to_list_node(Box::new(&[1, 2, 3, 9]));
        let l2 = array_to_list_node(Box::new(&[1, 2, 3]));

        let result = add_two_numbers(l1, l2);

        print_list_node(&result);

        let expr = array_to_list_node(Box::new(&[2, 4, 6, 9]));
        assert_eq!(result, expr);
    }
}
