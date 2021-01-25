/*

https://leetcode-cn.com/problems/re-space-lcci/


哦，不！你不小心把一个长篇文章中的空格、标点都删掉了，并且大写也弄成了小写。像句子"I reset the computer. It still didn’t boot!"已经变成了"iresetthecomputeritstilldidntboot"。在处理标点符号和大小写之前，你得先把它断成词语。当然了，你有一本厚厚的词典dictionary，不过，有些词没在词典里。假设文章用sentence表示，设计一个算法，把文章断开，要求未识别的字符最少，返回未识别的字符数。

注意：本题相对原题稍作改动，只需返回未识别的字符数



示例：

输入：
dictionary = ["looked","just","like","her","brother"]
sentence = "jesslookedjustliketimherbrother"
输出： 7
解释： 断句后为"jess looked just like tim her brother"，共7个未识别字符。
提示：

0 <= len(sentence) <= 1000
dictionary中总字符数不超过 150000。
你可以认为dictionary和sentence中只包含小写字母。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/re-space-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
1. 首先将dict转为map,加速查找.
使用动态规划思路:
i,j都是sentence中的下标
首先dp[i][j]表示从i到j(包含j)中未识别的字符数,显然这个数最大就是j-i+1
第一步,扫描整个字符串,找出所有包含在字典中的dp[i][j],然后设置为0
第二步,由短变长遍历整个dp,最终得出结果
空间复杂度是O(N^2),其中N是句子的长度
时间复杂度是O(N^3),其中N是句子的长度
*/

struct Solution {}
impl Solution {
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        if sentence.is_empty() {
            return 0;
        }
        let mut set = std::collections::HashSet::new();

        for s in dictionary.iter() {
            set.insert(s.as_str());
        }
        let s = sentence.as_str();
        let mut dp = vec![vec![0; sentence.len() + 1]; sentence.len() + 1];
        //第一步,扫描整个字符串,找出所有包含在字典中的dp[i][j],然后设置为0
        for i in 0..sentence.len() {
            for j in i..sentence.len() {
                if set.contains(&s[i..=j]) {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = j - i + 1;
                }
            }
        }
        // println!("dp={:?}", dp);
        //第二步,由短变长遍历整个dp,最终得出结果
        for step in 1..sentence.len() {
            for i in 0..sentence.len() - step {
                //从i开始往后走step步恰好是一个单词
                let total = &s[i..=(i + step)];
                if dp[i][i + step] == 0 {
                    continue;
                }
                // if total == "jesslooked" {
                //     println!("total={}", total);
                // }
                //没有组成一个单词,就开始切分,找出切分组合中最小的情况
                let mut expected = dp[i][i + step];
                for j in 0..step {
                    if dp[i][i + j] + dp[i + j + 1][i + step] < expected {
                        expected = dp[i][i + j] + dp[i + j + 1][i + step]
                    }
                }
                dp[i][i + step] = expected;
                // println!("total={},dp[{}][{}]={}", total, i, step, dp[i][step]);
            }
        }
        dp[0][sentence.len() - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::vec_str_to_string;
    #[test]
    fn it_works() {
        let a = Solution::respace(
            vec_str_to_string(vec!["looked", "just", "like", "her", "brother"]),
            "lookedjust".to_string(),
        );
        assert_eq!(a, 0);

        let a = Solution::respace(
            vec_str_to_string(vec!["looked", "just", "like", "her", "brother"]),
            "jesslooked".to_string(),
        );
        assert_eq!(a, 4);

        let a = Solution::respace(
            vec_str_to_string(vec!["looked", "just", "like", "her", "brother"]),
            "jesslookedjustliketimherbrother".to_string(),
        );
        assert_eq!(a, 7);

        let a = Solution::respace(
            vec_str_to_string(vec!["looked", "just", "like", "her", "brother"]),
            "".to_string(),
        );
        assert_eq!(a, 0);

        let a = Solution::respace(
            vec_str_to_string(vec!["a", "just", "like", "her", "brother"]),
            "a".to_string(),
        );
        assert_eq!(a, 0);
        return;
    }
}
