use std::fmt;

pub struct Stack<T: fmt::Display, const SIZE: usize> {
    data: [Option<T>; SIZE],
    top: isize,
}

impl<T: fmt::Display, const SIZE: usize> Stack<T, SIZE> {
    pub fn new() -> Self {
        return Self {
            data: [(); SIZE].map(|_| None),
            top: -1,
        };
    }

    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.top == (SIZE - 1) as isize {
            return Err("stack overflow");
        }
        self.top += 1;
        self.data[self.top as usize] = Some(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.top == -1 {
            return Err("stack underflow");
        }
        let idx = self.top as usize;
        let val = self.data[idx].take();
        self.top -= 1;
        Ok(val.unwrap())
    }
}

impl<T: fmt::Display, const SIZE: usize> fmt::Display for Stack<T, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..(self.top as usize + 1) {
            let elem = self.data[i].as_ref().unwrap();
            write!(f, "{}", elem)?;
            if i != (self.top as usize) {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        return fmt::Result::Ok(());
    }
}
