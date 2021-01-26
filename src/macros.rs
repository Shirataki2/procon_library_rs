#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}

#[macro_export]
macro_rules! max {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::max($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::max($a, max!($($rest),+)) };
}

#[macro_export]
macro_rules! chmax {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_max = max!($($rest),+);
        if $a < cmp_max { $a = cmp_max; true } else { false }
    }};
}

#[macro_export]
macro_rules! input { ($io:expr => $($name:ident: $t:ty),+) => { $(let $name: $t = $io.read();)* }; }

#[macro_export]
macro_rules! outln { ($io: expr) => { $io.write("\n".to_string()); }; ($io: expr => $fmt: expr) => {$io.write(format!(concat!($fmt, "\n")))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!(concat!($fmt, "\n"), $($arg)*)); }; }

#[macro_export]
macro_rules! out { ($io: expr => $fmt: expr) => {$io.write(format!($fmt, "\n"))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!($fmt, $($arg)*)); }; }

#[macro_export]
macro_rules! matrix {
    ($x: expr, $y: expr) => {
        Vector2d::new([$x, $y])
    };
    ($x: expr, $y: expr, $z: expr) => {
        Vector3d::new([$x, $y, $z])
    };
    ($w: expr, $x: expr, $y: expr, $z: expr) => {
        Vector4d::new([$w, $x, $y, $z])
    };
    ($($x: expr, $y: expr)=>*) => {
        [$(Vector2d::new([$x, $y])),*]
    };
    ($($x: expr, $y: expr, $z: expr)=>*) => {
        [$(Vector3d::new([$x, $y, $z])),*]
    };
    ($($w: expr, $x: expr, $y: expr, $z: expr)=>*) => {
        [$(Vector4d::new([$w, $x, $y, $z])),*]
    };
    ($($($x: expr),*);*) => {
        matrix!($($($x),*)=>*).into()
    };
}
