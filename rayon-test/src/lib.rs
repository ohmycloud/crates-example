#![feature(test)]
extern crate test;

use rayon::prelude::*;

pub struct Person {
    pub age: u32,
}

pub fn number_of_adults(people: &Vec<Person>) -> usize {
    people.iter().filter(|&p| p.age >= 18).count()
}

pub fn par_number_of_adults(people: &Vec<Person>) -> usize {
    people.par_iter().filter(|&p| p.age >= 18).count()
}

pub fn sum_of_add(input: &[i64]) -> i64 {
    input.iter().map(|&x| x + x).sum()
}

pub fn par_sum_of_add(input: &[i64]) -> i64 {
    input.par_iter().map(|&x| x + x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_number_of_adults(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let arr: Vec<Person> = (0..1_000_000)
            .map(|_| Person {
                age: rng.gen_range(13..80),
            })
            .collect();

        b.iter(|| black_box(number_of_adults(&arr)));
    }

    #[bench]
    fn bench_par_number_of_adults(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let arr: Vec<Person> = (0..1_000_000)
            .map(|_| Person {
                age: rng.gen_range(13..80),
            })
            .collect();

        b.iter(|| black_box(par_number_of_adults(&arr)));
    }

    #[bench]
    fn bench_sum_of_add(b: &mut Bencher) {
        let arr: Vec<i64> = (1..=10_000_000).collect();
        b.iter(|| black_box(sum_of_add(&arr)));
    }

    #[bench]
    fn bench_par_sum_of_add(b: &mut Bencher) {
        let arr: Vec<i64> = (1..=10_000_000).collect();
        b.iter(|| black_box(par_sum_of_add(&arr)));
    }
}
