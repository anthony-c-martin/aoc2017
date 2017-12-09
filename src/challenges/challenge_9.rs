pub fn execute(input: &str) {
    let (groups, garbage_chars) = count_groups(input);
    println!("Challenge 9a: {}", groups);
    println!("Challenge 9b: {}", garbage_chars);
}

fn count_groups(input: &str) -> (i32, i32) {
    let mut chars = input.chars().peekable();

    let mut depth = 0;
    let mut groups = 0;
    let mut garbage_chars = 0;
    let mut in_garbage = false;
    while let Some(&ch) = chars.peek() {
        match (ch, in_garbage) {
            ('{', false) => {
                depth += 1;
                groups += depth;
            },
            ('}', false) => {
                depth -= 1;
            },
            ('!', _) => {
                chars.next();
            },
            (',', false) => { ; },
            ('<', false) => {
                in_garbage = true;
            },
            ('>', true) => {
                in_garbage = false;
            },
            (_, false) => panic!("Got unexpected char '{}'.", ch),
            (_, true) => { garbage_chars += 1; }
        }

        chars.next();
    }

    (groups, garbage_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!((1,0), count_groups("{}"));
    }

    #[test]
    fn test_2() {
        assert_eq!((6,0), count_groups("{{{}}}"));
    }

    #[test]
    fn test_3() {
        assert_eq!((5,0), count_groups("{{},{}}"));
    }

    #[test]
    fn test_4() {
        assert_eq!((16,0), count_groups("{{{},{},{{}}}}"));
    }

    #[test]
    fn test_5() {
        assert_eq!((1,4), count_groups("{<a>,<a>,<a>,<a>}"));
    }

    #[test]
    fn test_6() {
        assert_eq!((9,8), count_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    }

    #[test]
    fn test_7() {
        assert_eq!((9,0), count_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    }

    #[test]
    fn test_8() {
        assert_eq!((3,17), count_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}