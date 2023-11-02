use pointer::{cons_list, demo, tree};

fn main() {
    demo();
    cons_list();

    let mut nums = Vec::new();
    for i in 0..=100 {
        nums.push(Some(i));
    }
    let root = tree::TreeNode::create(nums);

    println!("{}", tree::print_tree(root));
}
