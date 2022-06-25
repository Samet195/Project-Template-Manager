pub struct Library();
impl Library {
    pub fn main(&self) {
        println!("Hello, {}!", self.hello());
    }

    fn hello(&self) -> &str {
        "world"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library() {
        Library().main();
    }
}
