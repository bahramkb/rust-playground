pub struct MyMath {}

impl MyMath {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use crate::MyMath;

    #[test]
    fn it_works() {
        assert_eq!(MyMath::add(2, 2), 4);
    }
}
