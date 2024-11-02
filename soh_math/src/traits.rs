pub trait Convert<To> {
    fn convert(&self) -> To;
}
