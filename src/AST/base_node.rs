pub trait BaseNode<T> {
    fn evaluate(&self) -> T;
}
