pub mod trie {
    use std::marker::PhantomData;

    pub trait CharSet {
        fn char_size() -> usize;
        fn to_int(c: char) -> usize;
    }

    pub struct LargeAsciiCharSet {}
    impl CharSet for LargeAsciiCharSet {
        fn char_size() -> usize { 26 }
        fn to_int(c: char) -> usize {
            ((c as u32) - ('A' as u32)) as usize 
        }
    }

    pub struct SmallAsciiCharSet {}
    impl CharSet for SmallAsciiCharSet {
        fn char_size() -> usize { 26 }
        fn to_int(c: char) -> usize {
            ((c as u32) - ('a' as u32)) as usize 
        }
    }

    pub struct TrieNode<C> {
        next: Vec<isize>,
        exist: usize,
        accept: Vec<usize>,
        charset: PhantomData<C>,
    }
    impl<C: CharSet> TrieNode<C> {
        pub fn new() -> Self {
            let next = vec![-1; C::char_size()];
            Self {
                next,
                exist: 0,
                accept: vec![],
                charset: PhantomData
            }
        }
    }

    impl<C: CharSet> Default for TrieNode<C> {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct TrieTree<C> {
        nodes: Vec<TrieNode<C>>,
        pub root: usize,
    }

    impl<C: CharSet> TrieTree<C> {
        pub fn new() -> Self {
            let nodes = vec![TrieNode::new()];
            Self { nodes, root: 0 }
        }

        fn update_direct(&mut self, node: usize, id: usize) {
            self.nodes[node].accept.push(id);
        }

        fn update_child(&mut self, node: usize) {
            self.nodes[node].exist += 1;
        }

        pub fn add_inside(&mut self, s: &[char], str_idx: usize, node_idx: usize, id: usize) {
            if str_idx == s.len() {
                self.update_direct(node_idx, id);
            } else {
                let c = C::to_int(s[str_idx]);
                if self.nodes[node_idx].next[c] == -1 {
                    self.nodes[node_idx].next[c] = self.nodes.len() as isize;
                    self.nodes.push(TrieNode::new());
                }
                self.add_inside(s, str_idx + 1, self.nodes[node_idx].next[c] as usize, id);
                self.update_child(node_idx);
            }
        }

        pub fn add_at(&mut self, s: &[char], id: usize) {
            self.add_inside(s, 0, 0, id);
        }

        pub fn add(&mut self, s: &[char]) {
            self.add_at(s, self.nodes[0].exist);
        }

        pub fn query_at(&mut self, s: &[char], mut f: impl FnMut(&usize), str_idx: usize, node_idx: usize) {
            self.nodes[node_idx].accept.iter().for_each(|&idx| (f)(&idx));
            if str_idx != s.len() {
                let c = C::to_int(s[str_idx]);
                if self.nodes[node_idx].next[c] == -1 { return }
                self.query_at(s, f, str_idx + 1, self.nodes[node_idx].next[c] as usize)
            }
        }

        pub fn query(&mut self, s: &[char], f: impl FnMut(&usize)) {
            self.query_at(s, f, 0, 0);
        }

        pub fn count(&self) -> usize {
            self.nodes[0].exist
        }

        pub fn size(&self) -> usize {
            self.nodes.len()
        }
    }

    impl<C: CharSet> Default for TrieTree<C> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::trie::*;

    #[test]
    fn test_charset() {
        assert_eq!(LargeAsciiCharSet::to_int('A'), 0);
        assert_eq!(LargeAsciiCharSet::to_int('C'), 2);
        assert_eq!(LargeAsciiCharSet::to_int('Z'), 25);
        assert_eq!(SmallAsciiCharSet::to_int('a'), 0);
        assert_eq!(SmallAsciiCharSet::to_int('f'), 5);
    }

    #[test]
    fn test_tenka1() {
        let mut trie = TrieTree::<SmallAsciiCharSet>::new();
        let s = "abracadabra".chars().collect::<Vec<_>>();
        let p = vec!["b", "abra", "cad", "rac"];
        let w = vec![1, 10, 50, 100];
        p.iter().for_each(|&pi| trie.add(&pi.chars().collect::<Vec<_>>()));
        let mut dp = vec![0; s.len()+1];
        let dp_ref = &mut dp;
        for i in 0..s.len() {
            trie.query_at(&s, |&idx| {
                dp_ref[i + p[idx].len()] = std::cmp::max(dp_ref[i + p[idx].len()], dp_ref[i] + w[idx]);
            }, i, 0);
            dp_ref[i+1] = std::cmp::max(dp_ref[i + 1], dp_ref[i]);
        }
        assert_eq!(dp.last().unwrap(), &111);
    }
}