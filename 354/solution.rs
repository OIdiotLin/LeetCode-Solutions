struct Solution {}
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }
        let mut pairs = envelopes.clone();
        pairs.sort();
        let mut f = vec![1; pairs.len()];
        let mut res = 1;
        for i in 1..f.len() {
            for j in (0..i).rev() {
                if pairs[j][0] < pairs[i][0] && pairs[j][1] < pairs[i][1] {
                    f[i] = std::cmp::max(f[j] + 1, f[i]);
                }
            }
            res = std::cmp::max(res, f[i])
        }
        res
    }
}


fn main() {
    let vec = vec![vec![4,5],
                   vec![4,6],
                   vec![6,7],
                   vec![2,3],
                   vec![1,1]];
    println!("{}", Solution::max_envelopes(vec));
}
