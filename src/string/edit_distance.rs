use crate::*;

pub fn edit_distance<T: Eq>(s: &[T], t: &[T]) -> usize {
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![n+m+1; m+1]; n+1];
    dp[0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            if i < n { chmin!(dp[i+1][j], dp[i][j] + 1); }
            if j < m { chmin!(dp[i][j+1], dp[i][j] + 1); }
            if i < n && j < m {
                let c = if s[i] != t[j] { 1 } else { 0 };
                chmin!(dp[i+1][j+1], dp[i][j] + c);
            }
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance_string() {
        let s = "biting".chars().collect::<Vec<char>>();
        let t = "whiten".chars().collect::<Vec<char>>();
        assert_eq!(edit_distance(&s, &t), 4);
        let s = String::from("icpc").chars().collect::<Vec<char>>();
        let t = String::from("icpc").chars().collect::<Vec<char>>();
        assert_eq!(edit_distance(&s, &t), 0);
    }

    #[test]
    fn test_edit_distance_vec_abc185e() {
        let s = vec![1, 3, 2, 4];
        let t = vec![1, 5, 2, 6, 4, 3];
        assert_eq!(edit_distance(&s, &t), 3);
    }
}