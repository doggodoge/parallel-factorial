use ibig::{ubig, UBig};
use rayon::prelude::*;

/// Calculate a factorial number from an integer.
pub fn factorial(n: u64) -> String {
    let chunk_size = n as usize / rayon::current_num_threads();
    let vec = (1..=n).collect::<Vec<_>>();

    if n < (rayon::current_num_threads() as u64) * 2 {
        return single_threaded_factorial(n);
    }

    vec.chunks(chunk_size)
        .par_bridge()
        .into_par_iter()
        .map(apply_factorial_to_range)
        .collect::<Vec<UBig>>()
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap()
        .to_string()
}

fn apply_factorial_to_range(range: &[u64]) -> UBig {
    let mut acc = ubig!(1);
    for &number in range {
        acc *= number;
    }
    acc
}

fn single_threaded_factorial(n: u64) -> String {
    let mut acc = UBig::from(n);
    for index in 1..n {
        acc *= index;
    }
    acc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_should_be_correct_for_small_integer() {
        let result = single_threaded_factorial(3);
        assert_eq!(String::from("6"), result);
    }

    #[test]
    fn factorial_should_be_correct_for_large_integer() {
        let expected = "89461821307829752868514417153983165206980821677\
        9571907213868063227837990693501860533361810841010176000000000000000000";
        let actual = single_threaded_factorial(79);
        assert_eq!(expected, &actual);
    }

    #[test]
    fn parallel_factorial_should_be_correct_for_small_integer() {
        let actual = factorial(3);
        assert_eq!("6", &actual);
    }

    #[test]
    fn parallel_factorial_should_be_correct_for_large_integer() {
        let expected = "89461821307829752868514417153983165206980821677\
        9571907213868063227837990693501860533361810841010176000000000000000000";
        let actual = factorial(79);
        assert_eq!(expected, &actual);
    }
}
