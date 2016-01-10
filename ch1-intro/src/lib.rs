extern crate rand;

use rand::{Rand, Rng, thread_rng};

#[test]
fn test_sorted() {
    assert!(sorted(&mut vec!(1).into_iter()));
    assert!(sorted(&mut vec!(1,2).into_iter()));
    assert!(sorted(&mut vec!(1,2,100).into_iter()));
    assert!(!sorted(&mut vec!(2,1,100).into_iter()));
}

pub fn sorted<L: Iterator>(items: &mut L) -> bool where L::Item: Ord {
    match items.next() {
        None => return false,
        Some(mut prev) => for item in items {
            if item < prev {
                return false
            } else {
                prev = item
            }
        }
    }
    true
}

#[test]
fn test_random_vec() {
    let v: Vec<u64> = random_vec(0);
    assert!(v.len() == 0);

    let v: Vec<u64> = random_vec(100);
    assert!(v.len() == 100);
}

pub fn random_vec<R: Rand>(n: i64) -> Vec<R> {
    let mut v: Vec<R> = vec!();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        v.push(rng.gen());
    }
    v
}

/*
pub fn insertion_sort<A: Ord, Copy>(items: &mut Vec<A>) {
    let mut val: A;
    for i in 0..items.len() {
        val = items[i];
        for j in 0..i {
            if items[j] > val {
                shift(&mut items, j, i);
                items[j] = val;
            }
        }
    }
}
*/

#[test]
fn test_shift() {
    let v: Vec<i32> = vec!(100, 101, 102, 103, 104);
    shift(&mut v, 2, 5);
    assert!(v == vec!(100, 101, 102, 102, 103));
}

fn shift<A: Ord, Copy>(items: &mut Vec<A>, start: usize, end: usize) {
    for i in (start..end).rev() {
        items[i] = items[i-1];
    }
}
