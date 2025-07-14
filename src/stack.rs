
struct Stack<T, const SIZE: usize> {
    data: [T; SIZE],
    pointer: usize,
}