trait Number {
    fn i32_value(&self) -> i32;
}

impl Number for i32 {
    fn i32_value(&self) -> i32 {
        self.clone()
    }
}

#[cfg_attr(test, mockem::mock)]
fn generic_fn<T: Number>(x: T) -> i32 {
    x.i32_value()
}

#[test]
fn test_generic_fn() {
    use mockem::MockCall;

    let i = 0;
    generic_fn.mock_once(|_: i32| 10);
    let val = generic_fn(i);
    assert_eq!(val, 10);
}
