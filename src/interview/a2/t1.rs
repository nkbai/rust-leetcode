/*
给定一个正整数 n，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的正方形矩阵。

示例:

输入: 3
输出:
[
 [ 1, 2, 3 ],
 [ 8, 9, 4 ],
 [ 7, 6, 5 ]
]
*/
struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = Vec::new();
        for _ in 0..n {
            let mut v = Vec::new();
            for _ in 0..n {
                v.push(0);
            }
            matrix.push(v);
        }
        let mut dir = 0; //0 right, 1 down 2 left 3 top
        let mut si = 0;
        let mut sj = 0;
        let mut c = 1;
        matrix[0][0] = c;
        if n == 1 {
            return matrix;
        }
        let f = move || {
            loop {
                match dir {
                    0 => {
                        sj += 1;
                        if matrix[si as usize][sj as usize] != 0 {
                            return matrix;
                        }
                        //向左
                        while sj <= n - 1 && matrix[si as usize][sj as usize] == 0 {
                            c += 1;
                            matrix[si as usize][sj as usize] = c;
                            sj += 1;
                        }
                        sj -= 1;
                    }
                    1 => {
                        si += 1;
                        //走不下去了就结束
                        if matrix[si as usize][sj as usize] != 0 {
                            return matrix;
                        }
                        while si <= n - 1 && matrix[si as usize][sj as usize] == 0 {
                            c += 1;
                            matrix[si as usize][sj as usize] = c;
                            si += 1;
                        }
                        si -= 1;
                    }

                    2 => {
                        sj -= 1;
                        if matrix[si as usize][sj as usize] != 0 {
                            return matrix;
                        }
                        while sj >= 0 && matrix[si as usize][sj as usize] == 0 {
                            c += 1;
                            matrix[si as usize][sj as usize] = c;
                            sj -= 1;
                        }
                        sj += 1;
                    }
                    3 => {
                        si -= 1;
                        if matrix[si as usize][sj as usize] != 0 {
                            return matrix;
                        }
                        while si >= 0 && matrix[si as usize][sj as usize] == 0 {
                            c += 1;
                            matrix[si as usize][sj as usize] = c;
                            si -= 1;
                        }
                        si += 1;
                    }
                    _ => panic!("not possible"),
                }
                dir += 1;
                dir %= 4;
            }
        };
        return f();
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::generate_matrix(1);
        println!("t={:?}", t);
    }
}
