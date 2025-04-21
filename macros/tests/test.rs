#![allow(unexpected_cfgs)]

use alphabet_detector_macros::*;

#[derive(Debug, PartialEq)]
enum TestEnum {
    A,
    B,
}

#[test]
fn test() {
    let ch = 'a';
    let res: &[TestEnum] =
        alphabet_match!([(TestEnum::A, ['a', 'b', 'c']), (TestEnum::B, ['a', 'c']),]);
    assert_eq!(res, &[TestEnum::A, TestEnum::B]);

    let ch = 'b';
    let res: &[TestEnum] =
        alphabet_match!([(TestEnum::A, ['a', 'b', 'c']), (TestEnum::B, ['a', 'c']),]);
    assert_eq!(res, &[TestEnum::A]);

    let ch = 'd';
    let res: &[TestEnum] =
        alphabet_match!([(TestEnum::A, ['a', 'b', 'c']), (TestEnum::B, ['a', 'c']),]);
    assert_eq!(res, &[TestEnum::A, TestEnum::B]);
}
