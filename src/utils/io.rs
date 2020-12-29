pub mod io {
    use std::io::{Read, Write};
    pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

    impl<R, W> IO<R, W>
    where
        R: Read,
        W: Write,
    {
        /// To use standard I/O
        /// 
        /// `IO::new(std::io::stdin(), std::io::stdout())`
        pub fn new(r: R, w: W) -> Self {
            Self(r, std::io::BufWriter::new(w))
        }

        pub fn write<S: ToString>(&mut self, s: S) {
            self.1.write_all(s.to_string().as_bytes()).unwrap();
        }

        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            let buf = 
                self
                .0.by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\t' || b == b'\r')
                .take_while(|&b| b != b' ' && b != b'\n' && b != b'\t' && b != b'\r')
                .collect::<Vec<_>>();
            unsafe { std::str::from_utf8_unchecked(&buf) }
                .parse()
                .ok()
                .expect("Parse Error")
        }

        pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.read()).collect()
        }

        pub fn chars(&mut self) -> Vec<char> {
            self.read::<String>().chars().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::io::*;
    use crate::{input, outln, out};

    #[test]
    fn test_input() {
        let input = br"5
        10 20 15    30 5
        something  

        3.14   -10.7
        hoge";
        let mut sc = IO::new(&input[..], Vec::new());

        let n: usize = sc.read();
        assert_eq!(n, 5);

        let v: Vec<u32> = sc.vec(n);
        assert_eq!(v, vec![10, 20, 15, 30, 5]);
        
        let s = sc.chars();
        assert_eq!(s, vec!['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g']);
    
        let f: f64 = sc.read();
        assert_eq!(f, 3.14);

        let neg: f64 = sc.read();
        assert_eq!(neg, -10.7);
    }

    #[test]
    fn test_input_macro() {
        let input = br"5
        10 20 15    30 5
        something  

        3.14   -10.7
        hoge";
        let mut io = IO::new(&input[..], Vec::new());

        input!(io => n: usize);
        assert_eq!(n, 5);

        let v: Vec<u32> = io.vec(n);
        assert_eq!(v, vec![10, 20, 15, 30, 5]);
        
        let s = io.chars();
        assert_eq!(s, vec!['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g']);
    
        input!(io => f: f64, neg: f64);
        assert_eq!(f, 3.14);
        assert_eq!(neg, -10.7);
    }

    #[test]
    fn test_output() {
        let mut output = Vec::new();
        {
            let mut sc = IO::new(&b""[..], &mut output);
            sc.write(format!("{}\n", 1));
            sc.write(format!("{}\n", 9));
            sc.write(format!("{} {}\n", 3, 7));
        }
        let o = String::from_utf8(output).expect("No UTF-8");
        assert_eq!(&o, "1\n9\n3 7\n");
    }
    #[test]
    fn test_output_macro() {
        let mut output = Vec::new();
        {
            let mut io = IO::new(&b""[..], &mut output);
            outln!(io => "{}", 1);
            outln!(io => "{}", 9);
            out!(io => "{} {}\n", 3, 7);
        }
        let o = String::from_utf8(output).expect("No UTF-8");
        assert_eq!(&o, "1\n9\n3 7\n");
    }
}