
// a comment *bold*
/// a doc comment w/chars: +, -, *, /, %, ^, &, |, <, >, =, !, :, ,, ., ;,
fn main(x : Field, y : pub Field) {
    assert(x != y);
    let c = x + y;
    assert(c as u64 > x as u64);
    assert(c as u64 >= x as u64);
    assert(x as u64 < c as u64);
    assert(x as u64 <= c as u64);
    let string1 = "hello world + - * / % ^ & | < > = ! : , . ;";
    let string2 = " z / 3 == 4 ^ 1";
}

/**
 * a block comment with some things like +
 * and some more things: %, ^, &, |
 */

#[test]
fn test_main() {
    main(2, 1);
}

#[test]
fn test_main2() {
    main(5, 2);
}

#[test]
fn test_main3() {
    main(6, 1);
}








