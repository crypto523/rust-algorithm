/// Aliquot sum of a number is defined as the sum of the proper divisors of
/// a number, i.e. all the divisors of a number apart from the number itself
/// For example: The aliquot sum of 6 is (1 + 2 + 3) = 6, and that of 15 is
/// (1 + 3 + 5) = 9
/// Wikipedia article on Aliquot Sum: https://en.wikipedia.org/wiki/Aliquot_sum

pub fn aliquot_sum(number: u64) -> u64 {
    if number == 1 || number == 0 {
        return 0;
    }
    let mut sum: u64 = 0;

    for i in 1..(number / 2 + 1) {
        if number % i == 0 {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit_number() {
        assert_eq!(aliquot_sum(6), 6);
    }

    #[test]
    fn two_digit_number() {
        assert_eq!(aliquot_sum(15), 9);
    }

    #[test]
    fn three_digit_number() {
        assert_eq!(aliquot_sum(343), 57);
    }
}
