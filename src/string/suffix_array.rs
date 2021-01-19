//! SA-IS based
//! Verified [Library Checker](https://judge.yosupo.jp/submission/34125)

pub mod suffix_array {
    use std::cmp::*;

    pub struct SuffixArray(pub Vec<usize>);

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    enum CharType{ L, S }

    impl SuffixArray {
        pub fn new(s: &str) -> Self {
            let mut max_si = 0;
            let si = s.chars().map(|c| {
                let i = c as usize;
                max_si = max(max_si, i);
                i
            }).collect::<Vec<usize>>();
            Self(SuffixArray::sa_is(&si, max_si))
        }

        #[allow(clippy::needless_range_loop)]
        fn sa_is(si: &[usize], max_si: usize) -> Vec<usize> {
            let n = si.len();
            if n <= 2 {
                return match n {
                    0 => vec![],
                    1 => vec![0],
                    _ => {
                        if si[0] < si[1] {
                            vec![0, 1]
                        } else {
                            vec![1, 0]
                        }
                    }
                }
            }
            let mut ct = vec![CharType::L; n];
            for i in (0..n-1).rev() {
                ct[i] = match si[i].cmp(&si[i+1]) {
                    Ordering::Less    => CharType::S,
                    Ordering::Greater => CharType::L,
                    Ordering::Equal   => ct[i+1],
                }
            }
            let (mut l_ctr, mut s_ctr) = (vec![0; max_si+1], vec![0; max_si+1]);
            ct.iter().enumerate().for_each(|(i, &c)| {
                match c {
                    CharType::L => { l_ctr[si[i]] += 1; }
                    CharType::S => { s_ctr[si[i]] += 1; }
                }
            });
            let mut chr_ranges = vec![(0, 0); max_si+1];
            let mut last = 0;
            for c in 0..=max_si {
                let c_total = l_ctr[c] + s_ctr[c];
                if c_total != 0 {
                    chr_ranges[c] = (last, last + c_total);
                    last += c_total;
                }
            }

            let mut lms_idx = vec![0; n];
            let mut lms_ctr = 0;
            for i in 1..n {
                if ct[i-1] == CharType::L && ct[i] == CharType::S {
                    lms_ctr += 1;
                    lms_idx[i] = lms_ctr;
                }
            }
            let mut lms = Vec::with_capacity(lms_ctr);
            for i in 1..n { if lms_idx[i] != 0 { lms.push(i); }}
            let mut sa = vec![0; n];
            SuffixArray::induce(&mut sa, &si, &lms, &l_ctr, &chr_ranges);
            if lms_ctr > 0 {
                let mut sorted_lms = Vec::with_capacity(lms_ctr);
                for &i in &sa {
                    if lms_idx[i-1] != 0 {
                        sorted_lms.push(i-1);
                    }
                }
                let mut lms_part_nums = vec![0; lms_ctr];
                let mut max_lms_part_idx = 0;
                for i in 1..lms_ctr {
                    let lms1_start = sorted_lms[i-1];
                    let lms1_end = if lms_idx[lms1_start] == lms_ctr { n } else { lms[lms_idx[lms1_start]] };
                    let lms2_start = sorted_lms[i];
                    let lms2_end = if lms_idx[lms2_start] == lms_ctr { n } else { lms[lms_idx[lms2_start]] };
                    let is_same = if (lms1_end - lms1_start) != (lms2_end - lms2_start) {
                        false
                    } else {
                        let mut same = true;
                        for o in 0..(lms1_end - lms1_start) {
                            same = si[lms1_start + o] == si[lms2_start + o];
                            if !same { break; }
                        }
                        same
                    };
                    if !is_same { max_lms_part_idx += 1; }
                    lms_part_nums[lms_idx[sorted_lms[i]] - 1] = max_lms_part_idx;
                }
                let lms_part_sa = SuffixArray::sa_is(&lms_part_nums, max_lms_part_idx);
                for i in 0..lms_ctr {
                    sorted_lms[i] = lms[lms_part_sa[i]];
                }
                SuffixArray::induce(&mut sa, &si, &sorted_lms, &l_ctr, &chr_ranges);
            }
            sa.iter().map(|idx| idx - 1).collect()
        }

        fn induce(sa: &mut [usize], si: &[usize], lms: &[usize], l_ctr: &[usize], chr_ranges: &[(usize, usize)]) {
            let n = si.len();
            for v in sa.iter_mut() { *v = 0; }
            let mut initials = vec![0; n];
            let mut ct = vec![CharType::L; n];
            let mut checked = vec![false; l_ctr.len()];
            si.iter().for_each(|&c| {
                let (rs, re) = chr_ranges[c];
                let mut l_count = l_ctr[c];
                if !checked[c] {
                    for j in rs..re {
                        ct[j] = if l_count != 0 { CharType::L } else { CharType::S };
                        if l_count != 0 { l_count -= 1; }
                        initials[j] = c;
                    }
                    checked[c] = true;
                }
            });

            let mut chr_index = vec![std::usize::MAX; l_ctr.len()];
            for &i in lms.iter().rev() {
                let c = si[i];
                chr_index[c] = if chr_index[c] == std::usize::MAX {
                    chr_ranges[c].1
                } else {
                    chr_index[c]
                } - 1;
                sa[chr_index[c]] = i + 1;
            }
            let mut chr_insert_count = vec![0; l_ctr.len()];
            let c = si[n - 1];
            sa[chr_ranges[c].0 + chr_insert_count[c]] = n;
            chr_insert_count[c] += 1;
            for i in 0..n {
                if sa[i] < 2 { continue; }
                let target_idx = sa[i] - 2;
                let target_c = si[target_idx];
                let target_start_idx = chr_ranges[target_c].0;
                let to_idx = target_start_idx + chr_insert_count[target_c];
                if ct[to_idx] == CharType::L {
                    sa[to_idx] = target_idx + 1;
                    chr_insert_count[target_c] += 1;
                }
            }
            chr_insert_count = vec![0; l_ctr.len()];
            for i in (0..n).rev() {
                if sa[i] < 2 { continue; }
                let target_idx = sa[i] - 2;
                let target_c = si[target_idx];
                let target_end_idx = chr_ranges[target_c].1 - 1;
                let to_idx = target_end_idx - chr_insert_count[target_c];
                if ct[to_idx] == CharType::S {
                    sa[to_idx] = target_idx + 1;
                    chr_insert_count[target_c] += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::suffix_array::*;
    use rand::{distributions::Alphanumeric, Rng};

    fn rand_str(n: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .map(char::from)
            .collect()
    }

    fn naive_suffix_array(s: &str) -> Vec<usize> {
        let mut arr = (0..s.len()).collect::<Vec<usize>>();
        arr.sort_by(|&a, &b| {
            s[a..].cmp(&s[b..])
        });
        arr
    }

    #[test]
    fn test_simple_suffix_array() {
        let s = String::from("abracadabra");
        let a = SuffixArray::new(&s);
        assert_eq!(a.0, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
    }

    #[test]
    fn test_naive_suffix_array() {
        let s = String::from("abracadabra");
        let a = naive_suffix_array(&s);
        assert_eq!(a, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
    }

    #[test]
    fn test_random_string() {
        use std::time::Instant;
        vec![10usize, 30, 100, 300, 1000, 3000].iter().for_each(|& n| {
            let s = rand_str(n);
            let start = Instant::now();
            let naive = naive_suffix_array(&s);
            let end = start.elapsed();
            eprintln!("{:6}: Naive {} ns", n, end.as_nanos());
            let start = Instant::now();
            let sais = SuffixArray::new(&s);
            let end = start.elapsed();
            eprintln!("{:6}: SA-IS {} ns", n, end.as_nanos());
            assert_eq!(sais.0, naive);
        });
    }

    // #[bench]
    // fn bench_naive_suffix_array(b: &mut test::Bencher) {
    //     let s = rand_str(50000);
    //     b.iter(|| naive_suffix_array(&s));
    // }

    // #[bench]
    // fn bench_sais_suffix_array(b: &mut test::Bencher) {
    //     let s = rand_str(50000);
    //     b.iter(|| SuffixArray::new(&s));
    // }
}
