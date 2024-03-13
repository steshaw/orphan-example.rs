use std::fmt;

/*
impl ToString for Vec<char> {
    fn to_string(&self) -> String {
        String::from_iter(self)
    }
}
*/

// Wrapper for vec<char> to implement Display.
struct V {
    v: Vec<char>,
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self.v)
    }
}

fn main() {
    let v = V { v: vec!['a', 'b'] };
    println!("Hello {}", v);
}
