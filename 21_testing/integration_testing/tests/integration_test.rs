mod common;

#[test]
fn test_add() {
    // 使用共用模块。
    common::setup();
    assert_eq!(integration_testing::add(3, 2), 5);
}
