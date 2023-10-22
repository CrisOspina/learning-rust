mod panic;

fn main() {
    // panic::unrecoverable()
    // panic::backtrace()
    // panic::recoverable();
    // panic::recoverable_clear();
    panic::recover_with_expect();
}
