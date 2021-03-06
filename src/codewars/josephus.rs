fn josephus_survivor(person_count: i32, skip: i32) -> i32 {
    let mut index = 0;
    for i in 1..person_count + 1 {
        index = (index + skip) % i
    }
    return index + 1;
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::josephus::josephus_survivor;

    #[test_case(7, 3, 4)]
    #[test_case(11, 19, 10)]
    #[test_case(1, 300, 1)]
    #[test_case(2, 300, 1)]
    #[test_case(7, 300, 7)]
    #[test_case(300, 300, 265)]
    fn should_return_correct_survivor(n: i32, k: i32, expected: i32) {
        let actual = josephus_survivor(n, k);
        assert_eq!(actual, expected);
    }
}
