use std::{cell::RefCell, cmp::max, collections::VecDeque, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    1 + max(dfs(&node.left), dfs(&node.right))
                }
            }
        }

        dfs(&root)
    }
    // 通过数组反序列化生成一棵树
    pub fn create(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if nums.is_empty() {
            return None;
        }
        let size = nums.len();
        let mut index = 0;
        let root = Some(Rc::new(RefCell::new(Self::new(nums[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        while !queue.is_empty() {
            let q_size = queue.len();
            for _i in 0..q_size {
                if let Some(x) = queue.pop_front().flatten() {
                    let mut node = x.borrow_mut();
                    let lseq = 2 * index + 1;
                    let rseq = 2 * index + 2;
                    if lseq < size && nums[lseq].is_some() {
                        node.left = Some(Rc::new(RefCell::new(Self::new(nums[lseq].unwrap()))));
                        queue.push_back(node.left.clone());
                    }

                    if rseq < size && nums[rseq].is_some() {
                        node.right = Some(Rc::new(RefCell::new(Self::new(nums[rseq].unwrap()))));
                        queue.push_back(node.right.clone());
                    }
                }
                index += 1;
            }
        }
        root
    }

    // 将一棵树序列化成一个数组
    pub fn literal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let qsize = queue.len();
            for _ in 0..qsize {
                match queue.pop_front().flatten() {
                    Some(x) => {
                        ans.push(Some(x.borrow().val));
                        queue.push_back(x.borrow().left.clone());
                        queue.push_back(x.borrow().right.clone());
                    }
                    None => ans.push(None),
                }
            }
        }
        let size = ans.len();
        for i in (0..size).rev() {
            if ans[i].is_none() {
                ans.pop();
            } else {
                break;
            }
        }
        ans
    }
}

pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let height = TreeNode::get_height(&root);
    let width = (1 << height) - 1;
    let mut ans = vec![vec![" ".to_string(); width as usize]; height as usize];

    fn dfs(
        ans: &mut Vec<Vec<String>>,
        node: &Option<Rc<RefCell<TreeNode>>>,
        deep: usize,
        lo: usize,
        hi: usize,
    ) {
        if let Some(x) = node {
            let node = x.borrow();
            let mid = lo + (hi - lo) / 2;
            ans[deep][mid] = x.borrow().val.to_string();
            dfs(ans, &node.left, deep + 1, lo, mid);
            dfs(ans, &node.right, deep + 1, mid + 1, hi);
        }
    }

    dfs(&mut ans, &root, 0usize, 0usize, width as usize);
    ans.iter()
        .map(|x| x.concat())
        .collect::<Vec<_>>()
        .join("\n")
}
