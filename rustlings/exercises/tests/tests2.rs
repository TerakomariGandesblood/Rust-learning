// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    fn add_two(val: i32) -> i32 {
        val + 2
    }

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(add_two(2), 4);
    }
}
