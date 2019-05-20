struct Solution {}
impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return 0;
        }
        let mut f = vec![vec![0; b.len()+1]; a.len()+1];
        let mut res = 0;
        for i in 1..a.len()+1 {
            for j in 1..b.len()+1 {
                f[i][j] = if a[i-1] == b[j-1] {f[i-1][j-1] + 1} else {0};
                res = std::cmp::max(res, f[i][j])
            }
        }
        res
    }
}


fn main() {
    let v1 = vec![1,2,3,2,1];
    let v2 = vec![3,2,1,4,7];
    println!("{}", Solution::find_length(v1, v2));
}
