# Mock'em

[<img alt="github" src="https://img.shields.io/badge/github-poonesnerfect/mockem-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/poonesnerfect/mockem)
[<img alt="crates.io" src="https://img.shields.io/crates/v/mockem.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/mockem)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-mockem-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/mockem)

src crate: mockem: https://crates.io/crates/mockem

src repo: https://github.com/PoOnesNerfect/mockem

I add generic functions support and updated the crate.

Mock any function in Rust.

Make sure to only use this crate for testing purposes, as it will add a lot of overhead to your code.

## Basic Usage

On the function you want to mock, add the `#[mock]` attribute.

```rust
#[cfg_attr(test, mockem::mock)]
fn foo(a: &str) -> String {
    format!("{a}")
}

fn bar() -> String {
    format!("Hello, {}!", foo("bar"))
}

#[test]
fn test_fn() {
    use mockem::MockCall;

    foo.mock_once(|a| format!("mocked {a}"));

    assert_eq!(&bar(), "Hello, mocked bar!");

    // function works normally after mock return value is returned
    assert_eq!(&bar(), "Hello, bar!");
}
```

### Mocking Repeatedly

If you want to mock a function more than once or indefinitely, use `mock_repeat` instead of `mock_once`.

`mock_repeat` takes an `Option<usize>` as its first argument, which is the number of times to mock the function;

`None` means to mock the function indefinitely.


```rust
#[cfg_attr(test, mockem::mock)]
fn foo(a: &str) -> String {
    format!("{a}")
}

fn bar(a: &str) -> String {
    format!("Hello, {}!", foo(a))
}

#[test]
fn test_fn() {
    use mockem::{MockCall, ClearMocks};

    foo.mock_repeat(None, |a| format!("mocked {a}"));

    assert_eq!(&bar("bar"), "Hello, mocked bar!");
    assert_eq!(&bar("foo"), "Hello, mocked foo!");
    assert_eq!(&bar("baz"), "Hello, mocked baz!");

    // this clears all mocks, which will stop the indefinite mock
    foo.clear_mocks();

    assert_eq!(&bar("baz"), "Hello, baz!");
}
```


## Impl Blocks

If you want to mock impl methods, add the `#[mock]` attribute to the impl block.
Do the same for impl trait methods.

This will mock all methods in the impl block.

```rust
struct Foo;

#[cfg_attr(test, mockem::mock)]
impl Foo {
    fn foo(&self) -> String {
        format!("foo")
    }
}

trait Baz {
    fn baz(&self) -> String;
}

#[cfg_attr(test, mockem::mock)]
impl Baz for Foo {
    fn baz(&self) -> String {
        format!("baz")
    }
}

fn bar() -> String {
    format!("Hello, {} and {}!", Foo.foo(), Foo.baz())
}

#[test]
fn test_fn() {
    use mockem::MockCall;

    Foo::foo.mock_once(|_| "mockem".to_owned());
    Foo::baz.mock_once(|_| "mockem2".to_owned());

    assert_eq!(&bar(), "Hello, mockem and mockem2!");
}
```

## Async Functions

Async functions are also supported.

```rust
use async_trait::async_trait;

struct Foo;

#[cfg_attr(test, mockem::mock)]
impl Foo {
    async fn foo(&self) -> String {
        format!("foo")
    }
}

#[async_trait]
trait Baz {
    async fn baz(&self) -> String;
}

// also works with async_trait
// but you must place #[mock] above #[async_trait]
#[cfg_attr(test, mockem::mock)]
#[async_trait]
impl Baz for Foo {
    async fn baz(&self) -> String {
        format!("baz")
    }
}

async fn bar() -> String {
    format!("Hello, {} and {}!", Foo.foo().await, Foo.baz().await)
}

#[test]
fn test_fn() {
    use mockem::MockCall;
    
    Foo::foo.mock_once(|_| "mockem".to_owned());
    Foo::baz.mock_once(|_| "mockem2".to_owned());
    
    assert_eq!(&bar().await, "Hello, mockem and mockem2!");
}
```
## Generic Functions
```
trait Number{
    fn i32_value(&self) -> i32;
}

impl Number for i32{
    fn i32_value(&self) -> i32{
        self.clone()
    }
}


#[cfg_attr(test, mockem::mock)]
fn generic_fn<T: Number>(x: T) -> i32{
    x.i32_value()
}

#[test]
fn test_generic_fn(){
    use mockem::MockCall;

    let i = 0;
    generic_fn.mock_once(|_ : i32| {
        10
    });
    let val = generic_fn(i);
    assert_eq!(val, 10);
}

```

