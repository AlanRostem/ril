use std::fmt;

pub struct Stack<T: fmt::Display, const SIZE: usize> {
    data: [Option<T>; SIZE],
    pointer: usize,
}

impl<T: fmt::Display, const SIZE: usize> Stack<T, SIZE> {
    pub fn new() -> Self {
        return Self{
            data: [(); SIZE].map(|_| None),
            pointer: 0,
        };
    }

    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.pointer == SIZE {
            return Err("stack overflow");
        }
        self.data[self.pointer] = Some(value);
        self.pointer += 1;
        Ok(())
    }

    // TODO: implement pop
}

impl<T: fmt::Display, const SIZE: usize> fmt::Display for Stack<T, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.data.len() {
            let elem = self.data[i].as_ref().unwrap();
            write!(f, "{}", elem)?;
            if i != self.data.len()-1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        return fmt::Result::Ok(());
    }
}