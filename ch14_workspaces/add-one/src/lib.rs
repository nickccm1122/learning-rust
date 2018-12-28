use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn should_add_one() {
        assert_eq!(add_one(3), 4);
    }
}
