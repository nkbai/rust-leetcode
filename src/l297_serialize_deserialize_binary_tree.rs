/*
297. 二叉树的序列化与反序列化

序列化是将一个数据结构或者对象转换为连续的比特位的操作，进而可以将转换后的数据存储在一个文件或者内存中，同时也可以通过网络传输到另一个计算机环境，采取相反方式重构得到原数据。

请设计一个算法来实现二叉树的序列化与反序列化。这里不限定你的序列 / 反序列化算法执行逻辑，你只需要保证一个二叉树可以被序列化为一个字符串并且将这个字符串反序列化为原始的树结构。

示例:

你可以将以下二叉树：

    1
   / \
  2   3
     / \
    4   5

序列化为 "[1,2,3,null,null,4,5]"
提示: 这与 LeetCode 目前使用的方式一致，详情请参阅 LeetCode 序列化二叉树的格式。你并非必须采取这种方式，你也可以采用其他的方法解决这个问题。

说明: 不要使用类的成员 / 全局 / 静态变量来存储状态，你的序列化和反序列化算法应该是无状态的。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/serialize-and-deserialize-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
序列化:
按照前序遍历规则填充,无论是否有有效子节点都填充子节点,需要确定的是
1. 这一层是否有子节点
2. 这一层最后一个子节点的位置
反序列化:
共享中的build_tree就是例子
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
#[derive(PartialEq, Eq, Clone, Debug)]
struct SerializeLayer {
    last_pos: i32,                             //-1表示这一层没有任何节点
    nodes: Vec<Option<Rc<RefCell<TreeNode>>>>, //这一层的所有节点
}
impl SerializeLayer {
    fn new() -> SerializeLayer {
        SerializeLayer {
            last_pos: -1,
            nodes: Vec::new(),
        }
    }
}
impl Solution {
    /*
        1
       / \
      2   3
         / \
        4   5
    会序列化为[1,2,3,NULL,NULL,4,5]
    但是这棵树:
        1
       / \
      2   3
         / \
        4   5
             \
              6
    则会序列化为[1,2,3,NULL,NULL,4,5,NULL,NULL,NULL,NULL,NULL,NULL,NULL,6]
    是存在优化空间的
        */
    pub fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut layer = SerializeLayer::new();
        if root.is_none() {
            return String::new();
        }
        let mut r = String::new(); //结果不存在这里

        //第一层
        layer.nodes.push(root.clone());
        layer.last_pos = 0;
        while layer.last_pos >= 0 {
            //            println!("layer={:?}", layer);
            let mut next_layer = SerializeLayer::new();
            let mut i = 0;
            while i <= layer.last_pos {
                let cn = layer.nodes[i as usize].clone();
                if cn.is_none() {
                    r += &format!("{},", "NULL").to_string();
                } else {
                    let n: Rc<RefCell<TreeNode>> = cn.unwrap();
                    r = r + &format!("{},", n.borrow().val).to_string();
                    //左右节点都填充上相应的值,无论是否有效,不能空
                    if n.borrow().left.is_none() {
                        next_layer.nodes.push(None);
                    } else {
                        next_layer.last_pos = 2 * i;
                        next_layer.nodes.push(n.borrow().left.clone());
                    }
                    if n.borrow().right.is_none() {
                        next_layer.nodes.push(None);
                    } else {
                        next_layer.last_pos = 2 * i + 1;
                        next_layer.nodes.push(n.borrow().right.clone());
                    }
                }
                i += 1;
            }
            //切换到下一层
            layer = next_layer;
        }
        //移除最后一个,
        r.remove(r.len() - 1);
        r
    }
    pub fn deserialize(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let ss: Vec<_> = s.split(",").collect();
        if ss.len() <= 0 {
            return None;
        }
        //        println!("ss={:?}", ss);
        Solution::build_tree_helper(0, &ss)
    }
    fn build_tree_helper(i: usize, v: &Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        //        println!("i={}", i);
        if i >= v.len() {
            return None;
        }
        if v[i] == "NULL" {
            return None;
        }
        let left = Self::build_tree_helper(2 * i + 1, v);
        let right = Self::build_tree_helper(2 * i + 2, v);
        Some(Rc::new(RefCell::new(TreeNode {
            val: v[i].parse().unwrap(), //错误就直接崩溃了,实际项目中肯定不能这么做
            left,
            right,
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_serialize() {
        println!("tree={:?}", build_tree(&vec![0, -3, 9, -10, NULL, 5]));
        println!(
            "r={}",
            Solution::serialize(build_tree(&vec![0, -3, 9, -10, NULL, 5]))
        );
        let t = build_tree(&vec![0, -3, 9, -10, NULL, 5]);
        assert_eq!(t, Solution::deserialize(Solution::serialize(t.clone())));
    }
}
