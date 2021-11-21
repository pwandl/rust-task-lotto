use itertools::Itertools;
use std::env;

use rand::{prelude::IteratorRandom, thread_rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut rng = thread_rng();
        Lotto {
            take,
            from,
            numbers: (1..=from).choose_multiple(&mut rng, take),
        }
    }

    fn get_numbers(&self) -> Vec<usize> {
        self.numbers.clone()
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    format!(
        "{} of {}: [{}]",
        lotto.take,
        lotto.from,
        lotto.get_numbers().iter().format(", ")
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || (args.len() % 2) != 1 {
        println!("Invalid number of arguments");
        return;
    }
    for i in 0..args.len() / 2 {
        let take: usize = args[2 * i + 1].parse().expect("Could not parse take");
        let from: usize = args[2 * i + 2].parse().expect("Could not parse from");
        let lotto = Lotto::new(take, from);
        println!("{}", format_lotto_results(&lotto))
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
