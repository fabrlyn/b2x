// question for reddit

pub trait ToSomething<T> {
    fn to_something(self) -> T;
}

pub trait ToOutput {
    type Output;
}

impl ToSomething<i32> for u8 {
    fn to_something(self) -> i32 {
        self as i32
    }
}

impl ToSomething<u32> for u8 {
    fn to_something(self) -> u32 {
        self as u32
    }
}

impl ToSomething<i64> for i32 {
    fn to_something(self) -> i64 {
        self as i64
    }
}

fn run() {
    let a: i32 = 10u8.to_something();
    let a = a.to_something();
}

// Whats up with this?
// let a = i8::from_str_radix("11111111", 2);
