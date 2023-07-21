impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;

        let mut curr1 = l1;
        let mut curr2 = l2;

        let mut start : Option<Box<ListNode>> = None;
        let mut curr : &mut Option<Box<ListNode>> = &mut None;

        while curr1.is_some() || curr2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(unwrap) = curr1 {
                sum += unwrap.val;
                curr1 = unwrap.next;
            }
            if let Some(unwrap) = curr2 {
                sum += unwrap.val;
                curr2 = unwrap.next;
            }

            carry = sum / 10;
            sum %= 10;

            if curr.is_none() {
                start = Some(Box::new(ListNode::new(sum)));
                curr = &mut start;
            } else {
                let new_node = Some(Box::new(ListNode::new(sum)));
                let curr_mut = curr.as_mut().unwrap();
                curr_mut.next = new_node;
                curr = &mut curr_mut.next;
            }
        }

        start
    }
}
