// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut end = &mut result;
    let mut carry = 0;

    fn extract(list_node: &mut Option<Box<ListNode>>) -> i32 {
        match list_node {
            Some(node) => {
                let val = node.val;
                *list_node = node.next.take();
                val
            }
            None => 0,
        }
    }

    while l1.is_some() || l2.is_some() {
        let lhs = extract(&mut l1);
        let rhs = extract(&mut l2);
        let tmp_res = lhs + rhs + carry;
        carry = tmp_res / 10;

        *end = Some(Box::new(ListNode::new(tmp_res % 10)));
        end = &mut end.as_deref_mut().unwrap().next;
    }

    if carry != 0 {
        *end = Some(Box::new(ListNode::new(carry)));
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn traverse_list(node: Option<Box<ListNode>>, list: &mut Vec<i32>) -> Vec<i32> {
        if let Some(list_node) = node {
            let ListNode { val, next } = list_node.as_ref();
            list.push(*val);
            traverse_list(next.clone(), list);
        }

        list.to_vec()
    }

    fn insert_node(prev_node: &mut Option<Box<ListNode>>, node: Option<Box<ListNode>>) {
        if let Some(list_node) = prev_node {
            let ListNode { val: _, next } = list_node.as_mut();
            if next.is_none() {
                *next = node
            } else {
                insert_node(next, node);
            }
        }
    }

    #[test]
    fn example_1() {
        let mut vec: Vec<i32> = vec![];
        let mut vec2: Vec<i32> = vec![];
        let mut vec3: Vec<i32> = vec![];
        let mut list = Some(Box::new(ListNode::new(9)));
        insert_node(&mut list, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list, Some(Box::new(ListNode::new(9))));

        let mut list2 = Some(Box::new(ListNode::new(9)));
        insert_node(&mut list2, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list2, Some(Box::new(ListNode::new(9))));
        insert_node(&mut list2, Some(Box::new(ListNode::new(9))));

        assert_eq!(
            vec![9, 9, 9, 9, 9, 9, 9],
            traverse_list(list.clone(), &mut vec)
        );
        assert_eq!(vec![9, 9, 9, 9], traverse_list(list2.clone(), &mut vec2));
        assert_eq!(
            vec![8, 9, 9, 9, 0, 0, 0, 1],
            traverse_list(add_two_numbers(list, list2), &mut vec3)
        );
    }
}
