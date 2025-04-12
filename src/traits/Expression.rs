pub trait Expression {
    fn accept(&self) -> String;
}
