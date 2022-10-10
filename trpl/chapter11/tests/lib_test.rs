// 集成测试（integration tests）
// Cargo 会将 tests 文件夹内每一个文件当作单独的 crate 来编译

// cargo test it_adds_two
// cargo test --test lib_test

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, chapter11::add(2, 2));
}
