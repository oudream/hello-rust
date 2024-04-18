// 定义树的节点结构体
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // 便利函数，用于创建新的树节点
    fn new(value: i32) -> TreeNode {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 向树中添加节点的函数，这里简单实现为小于等于当前节点值的放左边，大于的放右边
    fn insert(&mut self, value: i32) {
        if value <= self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
}

pub fn hello_box1() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    println!("Root: {}", root.value);
    if let Some(left) = root.left {
        println!("Left child: {}", left.value);
    }
    if let Some(right) = root.right {
        println!("Right child: {}", right.value);
    }
}
