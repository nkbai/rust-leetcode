#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
959. 由斜杠划分区域

在由 1 x 1 方格组成的 N x N 网格 grid 中，每个 1 x 1 方块由 /、\ 或空格构成。这些字符会将方块划分为一些共边的区域。

（请注意，反斜杠字符是转义的，因此 \ 用 "\\" 表示。）。

返回区域的数目。



示例 1：

输入：
[
" /",
"/ "
]
输出：2
解释：2x2 网格如下：

示例 2：

输入：
[
" /",
"  "
]
输出：1
解释：2x2 网格如下：

示例 3：

输入：
[
"\\/",
"/\\"
]
输出：4
解释：（回想一下，因为 \ 字符是转义的，所以 "\\/" 表示 \/，而 "/\\" 表示 /\。）
2x2 网格如下：

示例 4：

输入：
[
"/\\",
"\\/"
]
输出：5
解释：（回想一下，因为 \ 字符是转义的，所以 "/\\" 表示 /\，而 "\\/" 表示 \/。）
2x2 网格如下：

示例 5：

输入：
[
"//",
"/ "
]
输出：3
解释：2x2 网格如下：



提示：

1 <= grid.length == grid[0].length <= 30
grid[i][j] 是 '/'、'\'、或 ' '。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/regions-cut-by-slashes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
每个格子最多分成两份,要么左上,右下,要么左下,右上
对任意一个格子都进行编码给两个编号,
这样一个nxn的格子就有nxnx2个编号,
然后来看看如何处理:
1. 如果一个格子没有斜杠,那么他的0,1两个编号就可以merge
    如果右侧是空格,则自己的1号和右侧的0号merge
    如果右侧是/,则自己的1号和右侧的0号merge
    如果右侧是\,则自己的1号和右侧的0号merge
    如果下边是空格,则自己的0号和下边的1号merge
     如果下边是\,在自己的0号和下边的1号merge
    如果下边是/,则自己的0号和下边的0号merge

2. 如果一个格子是/,
    那么他的1号可以和右侧的格子的0号merge
    如果下侧是空格,则他自己的1号可以和下边的1号merge
    如果下边是\,则他自己的1号可以和下面的1号merge
    如果下边是/,则他自己的0号可以和下面的0号merge
3. 如果一个格子是\
  那么他自己的1号可以和右侧的格子的0号merge
    如果下边是空格,则他自己的0号可以和下边的0号merge
    如果下边是\,则他自己的0号可以和下边的1号merge
    如果下边是/,则他自己的0号可以和下边的0号merge

 **总结如下:**
 一个格子是空格,则他自己的0,1可以merge
 然后一个格子的1号总是可以和右侧格子的0号merge
 麻烦的是和下边格子的合并:
 简化处理一下,将自己是空格认定为和/一样,那么可以简化如下:
 如果一个格子是/:
    如果下侧是空格,则他自己的1号可以和下边的1号merge
    如果下边是\,则他自己的1号可以和下面的1号merge
    如果下边是/,则他自己的0号可以和下面的0号merge
 如果一个格子是\:
     如果下边是空格,则他自己的0号可以和下边的0号merge
    如果下边是\,则他自己的0号可以和下边的1号merge
    如果下边是/,则他自己的0号可以和下边的0号merge
*/
struct Solution {}
struct DSU {
    pre: Vec<i32>,
}
impl DSU {
    fn new(n: usize) -> DSU {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i as i32);
        }
        DSU { pre: v }
    }
    fn find(&mut self, x: i32) -> i32 {
        if self.pre[x as usize] == x {
            return x;
        }
        // let prex = self.pre[x];
        let prex = self.find(self.pre[x as usize]);
        //因为递归,这里会把一串上面的所有路径都压缩了,
        self.pre[x as usize] = prex;
        return prex;
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: i32, y: i32) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        //注意这里是设置的是prex的parent,而不是x的parent
        self.pre[prey as usize] = prex;
        return true;
    }
}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        const SPC: u8 = ' ' as u8;
        const SLASH: u8 = '/' as u8;
        const BACK_SLASH: u8 = '\\' as u8;

        let mut g = vec![];
        for s in grid {
            let s = s.as_bytes();
            let mut v = vec![];
            for c in s {
                v.push(*c);
            }
            g.push(v);
        }
        let row = g.len();
        let col = g[0].len();
        let mut dsu = DSU::new(row * col * 2);
        for i in 0..row {
            for j in 0..col {
                /*
                 一个格子是空格,则他自己的0,1可以merge
                 然后一个格子的1号总是可以和右侧格子的0号merge
                 麻烦的是和下边格子的合并:
                 简化处理一下,将自己是空格认定为和/一样,那么可以简化如下:
                 如果一个格子是/:
                    如果下侧是空格,则他自己的1号可以和下边的1号merge
                    如果下边是\,则他自己的1号可以和下面的1号merge
                    如果下边是/,则他自己的1号可以和下面的0号merge
                 如果一个格子是\:
                     如果下边是空格,则他自己的0号可以和下边的0号merge
                    如果下边是\,则他自己的0号可以和下边的1号merge
                    如果下边是/,则他自己的0号可以和下边的0号merge
                */
                let c = g[i][j];
                let i0 = i * col * 2 + j * 2;
                let i1 = i0 + 1;
                if c == SPC {
                    dsu.merge(i0 as i32, i1 as i32);
                }
                if j < col - 1 {
                    let r0 = i * col * 2 + j * 2 + 2;
                    dsu.merge(i1 as i32, r0 as i32);
                }
                if i < row - 1 {
                    let d0 = (i + 1) * col * 2 + j * 2;
                    let d1 = d0 + 1;
                    let d = g[i + 1][j];
                    if c == SLASH {
                        if d == SLASH {
                            dsu.merge(i1 as i32, d0 as i32);
                        } else {
                            dsu.merge(i1 as i32, d1 as i32);
                        }
                    } else {
                        if d == BACK_SLASH {
                            dsu.merge(i0 as i32, d1 as i32);
                        } else {
                            dsu.merge(i0 as i32, d0 as i32);
                        }
                    }
                }
            }
        }
        let mut map: Vec<i32> = vec![0; dsu.pre.len()];
        let mut count = 0;
        for i in 0..dsu.pre.len() {
            let x = dsu.find(i as i32);
            // println!("{}->{}", i, x);
            if map[x as usize] == 0 {
                count += 1;
            }
            map[x as usize] += 1;
        }
        count
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let r = Solution::regions_by_slashes(vec![" /".to_string(), "/ ".to_string()]);
        assert_eq!(r, 2);
        let r = Solution::regions_by_slashes(vec![" /".into(), "  ".into()]);
        assert_eq!(r, 1);
        let r = Solution::regions_by_slashes(vec!["\\/".into(), "/\\".into()]);
        assert_eq!(r, 4);
        let r = Solution::regions_by_slashes(vec!["/\\".into(), "\\/".into()]);
        assert_eq!(r, 5);
        let r = Solution::regions_by_slashes(vec!["//".into(), "/ ".into()]);
        assert_eq!(r, 3);
        let r = Solution::regions_by_slashes(vec!["  /".into(), "/ /".into(), "\\/ ".into()]);
        assert_eq!(r, 3);
    }
}
