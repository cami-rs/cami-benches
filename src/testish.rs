use core::ops::{Coroutine, CoroutineState};
use core::pin::Pin;

fn main() {
    let mut generator = #[coroutine]
    || {
        println!("Before yield 1");
        yield 1;
        println!("Before yield 2");
        yield 2;
        println!("Before return");
        return "foo";
    };

    match Pin::new(&mut generator).resume(()) {
        CoroutineState::Yielded(1) => {
            println!("Yielded 1");
        }
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        CoroutineState::Yielded(2) => {
            println!("Yielded 2");
        }
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        CoroutineState::Complete("foo") => {
            println!("Returned foo");
        }
        _ => panic!("unexpected value from resume"),
    }
}
