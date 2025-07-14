use std;
use std::fmt;


struct MyArr {
    pub data: [i32; 4]
}

fn main() {
    let other: MyArr = MyArr { data: [1, 2, 3, 4] };
    println!("Hello {}", other);
}

impl fmt::Display for MyArr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "[");
        res.expect("Failed to print opening bracket");
        for (i, elem) in self.data.iter().enumerate() {
            res = write!(f, "{}", elem);
            res.expect("Failed to write number");
            if i != self.data.len()-1 {
                res = write!(f, ", ");
                res.expect("Failed to write comma")
            }
        }
        res = write!(f, "]");
        res.expect("Failed to print closing bracket");
        return fmt::Result::Ok(());
    }
}
