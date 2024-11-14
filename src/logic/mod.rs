pub trait State<I, O> {
    fn init(&mut self);
    fn get(&mut self, input: &I) -> O;
}
