pub fn z_algorithm(s: &[char]) -> Vec<usize> {
    let mut z = vec![0; s.len()];
    let mut j = 0;
    for i in 1..s.len() {
        if i + z[i - j] < j + z[j] {
            z[i] = z[i - j];
        } else {
            let mut k = if j + z[j] > i { j + z[j] - i } else { 0 };
            while i + k < s.len() && s[k] == s[i + k] { k += 1; }
            z[i] = k;
            j = i;
        }
    }
    z[0] = s.len();
    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_algorithm() {
        let s = "ababa".chars().collect::<Vec<_>>();
        let z = z_algorithm(&s);
        assert_eq!(z, vec![5, 0, 3, 0, 1]);
    }
}