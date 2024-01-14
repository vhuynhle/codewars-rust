pub fn print(n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 {
        return None;
    }

    let n: usize = n.try_into().unwrap();
    let d = n / 2;
    let res: String = (0..=d)
        .chain((0..d).rev())
        .fold(String::new(), |mut output, l| {
            (0..d - l).for_each(|_| output.push(' '));
            (0..2 * l + 1).for_each(|_| output.push('*'));
            output.push('\n');
            output
        });
    Some(res)
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
