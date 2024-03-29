// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    // 1 mutable borrow, OR many read borrow
    let mut x = 100;

    let y = &mut x;
    *y += 100;

    let z = &mut x;
    *z += 1000;

    // gabsia dipake yang y lagi
    // *y += 10000;

    assert_eq!(x, 1200);
}
