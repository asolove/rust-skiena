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

fn test_insertion_sort_case<A>(actual: &mut Vec<A>, expected: &Vec<A>) where A: Ord, A: Copy, A: PartialEq {
    insertion_sort(actual);
    assert!(actual == expected);
}

#[test]
fn test_insertion_sort() {
    let mut empty: Vec<usize> = vec!();
    test_insertion_sort_case(&mut empty, &vec!());
    test_insertion_sort_case(&mut vec!(0), &vec!(0));
    test_insertion_sort_case(&mut vec!(0, 1), &vec!(0, 1));
    test_insertion_sort_case(&mut vec!(0, -1), &vec!(-1, 0));
    test_insertion_sort_case(&mut vec!(5, 1, 2, 4, 3), &vec!(1, 2, 3, 4, 5));
}

pub fn insertion_sort<A>(items: &mut Vec<A>) where A: Ord, A: Copy {
    let mut val: A;
    for i in 0..items.len() {
        val = items[i];
        for j in 0..i {
            if items[j] > val {
                shift(items, j, i);
                break;
            }
        }
    }
}

#[test]
fn test_shift() {
    let mut v: Vec<i32> = vec!(100, 101, 102, 103, 104);
    shift(&mut v, 2 as usize, 4 as usize);
    assert!(v == vec!(100, 101, 104, 102, 103));
}

// Moves the elements from start to end one slot to the right,
// and puts the element in end at start.
fn shift<A: Copy>(items: &mut Vec<A>, start: usize, end: usize) {
    let end_item = items[end];
    for i in (start..end).rev() {
        items[i+1] = items[i];
    }
    items[start] = end_item;
}
