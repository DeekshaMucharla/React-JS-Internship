use std::io;

// Define the binary tree structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor for TreeNode
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn construct_tree() -> Option<Box<TreeNode>> {
    println!("Enter the value of the root node:");
    let root_val: i32 = read_input();

    let mut root = Some(Box::new(TreeNode::new(root_val)));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(&mut root);

    while let Some(node) = queue.pop_front() {
        let node = match node {
            Some(n) => n,
            None => continue,
        };

        println!("Enter the value of the left child of node {} (or enter -1 for no child):", node.val);
        let left_val: i32 = read_input();
        if left_val != -1 {
            let left_child = Some(Box::new(TreeNode::new(left_val)));
            node.left = left_child;
            queue.push_back(&mut node.left);
        }

        println!("Enter the value of the right child of node {} (or enter -1 for no child):", node.val);
        let right_val: i32 = read_input();
        if right_val != -1 {
            let right_child = Some(Box::new(TreeNode::new(right_val)));
            node.right = right_child;
            queue.push_back(&mut node.right);
        }
    }

    root
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

fn main() {
    println!("Construct the binary tree:");
    let root = construct_tree();
    let depth = max_depth(&root);
    println!("The maximum depth of the binary tree is: {}", depth);
}
