mod stack;

fn main() -> Result<(), ()> {
    let mut s = stack::Stack::<i32, 4>::new();
    let _ = s.push(1);
    _ = s.push(2);
    _ = s.push(3);
    _ = s.push(4);
    println!("Hello {}", s);
    let res = s.pop();
    println!("value popped: {}", res.unwrap());
    println!("Hello {}", s);
    Ok(())
}


