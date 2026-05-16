// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut first_number_str = String::new();
    let mut second_number_str = String::new();

    let mut curr = l1;
    while let Some(node) = curr.clone() {
      let digit = node.val;
      first_number_str = format!("{}{}", first_number_str, digit);
      curr = node.next;
    }

    curr = l2;
    while let Some(node) = curr.clone() {
      let digit = node.val;
      second_number_str = format!("{}{}", second_number_str, digit);
      curr = node.next;
    }

    let first_number: i32 = first_number_str.parse().expect("Not a number!");
    let second_number: i32 = second_number_str.parse().expect("Not a number!");

    let result = first_number + second_number;

    build_list_from_number(result)
  }
}

/// Build singly linked list from number (most significant digit first)
pub fn build_list_from_number(mut n: i32) -> Option<Box<ListNode>> {
  if n == 0 {
    return Some(Box::new(ListNode::new(0)));
  }

  // Handle negative numbers by taking absolute value (you can adjust if needed)
  let is_negative = n < 0;
  if is_negative {
    n = -n;
  }

  // Collect digits in reverse order first
  let mut digits = Vec::new();
  while n > 0 {
    digits.push((n % 10) as i32);
    n /= 10;
  }

  // Build the list from back to front (so head has the highest digit)
  let mut head = None;
  for &digit in digits.iter().rev() {
    let mut node = ListNode::new(digit);
    node.next = head;
    head = Some(Box::new(node));
  }

  head
}

fn main() {
    println!("Hello, world!");
}
