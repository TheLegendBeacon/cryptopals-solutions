mod set1;

pub use set1::test_set1 as set1;

pub fn test_equal(message: &str, condition: bool) {
    print!("{} ", message);
    if condition {
        println!("\u{001b}[32;1mPASS\u{001b}[0m");
    } else {
        println!("\u{001b}[31;1mFAIL\u{001b}[0m")
    }
}