use loops::farenheit_to_celsius;
use loops::fib;

#[test]
fn farenheit_to_celsius_conversion() {
    assert_eq!(farenheit_to_celsius(95.0), 35.0);
    assert_eq!(farenheit_to_celsius(32.0), 0.0);
}

#[test]
fn fib_test() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    assert_eq!(fib(10), 55);
}
