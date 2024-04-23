#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut r = &mut list1;
    while list2.is_some() {
        if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
            std::mem::swap(r, &mut list2);
        }
        r = &mut r.as_mut()?.next;
    }
    list1
}

#[cfg(test)]
mod test {
    use crate::algos::leetcode::{merge_two_lists, merge_two_sorted_lists::ListNode};

    #[test]
    fn merge_to_lists_test() {
        assert_eq!(
            merge_two_lists(
                Some(Box::new(ListNode::new(10))),
                Some(Box::new(ListNode::new(20))),
            ),
            Some(Box::new(ListNode::new(20))),
        );
    }
}
