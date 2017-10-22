// Copyright © 2017 Bart Massey

// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// k-ary heapsort

extern crate pcg_rand;
extern crate rand;
extern crate criterion;
use criterion::Criterion;

use rand::Rng;
use pcg_rand::Pcg32;

#[cfg(test)]
const K: usize = 1024;
#[cfg(test)]
const N_BENCH: u32 = 128*K as u32;

// Benchmark sort timings, 1M in-order entries:
// D=8, 0.15s
// D=4, 0.12s
// D=2, 0.14s
const D: usize = 2;

fn unsorted(n: usize) -> Vec<u32> {
    let mut pcg = Pcg32::new_unseeded();
    let mut a: Vec<u32> = Vec::with_capacity(n);
    // XXX First call always returns 0. This seems like a bug.
    let _: u32 = pcg.gen();
    for _ in 0..n {
        a.push(pcg.gen())
    };
    a
}

fn downheap(a: &mut[u32], i0: usize) {
    let mut i = i0;
    loop {
        let mut m = i;
        for j in D*i+1..D*i+1+D {
            if j >= a.len() {
                break
            };
            if a[j] > a[m] {
                m = j
            };
        };
        if m == i {
            return
        };
        a.swap(i, m);
        i = m;
    }
}

fn heapify(a: &mut[u32]) {
    for i in (0..a.len()).rev() {
        downheap(a, i)
    }
}

#[cfg(test)]
fn checkheap(a: &[u32], i: usize) {
    for j in D*i+1..D*i+1+D {
        if j >= a.len() {
            return
        };
        assert!(a[j] <= a[i]);
        checkheap(a, j)
    }
}

#[test]
fn test_heapify() {
    let mut a = unsorted(3*D+17);
    heapify(&mut a);
    checkheap(&a, 0)
}

fn heapsort(a: &mut[u32]) {
    heapify(a);
    for i in (1..a.len()).rev() {
        a.swap(0, i);
        downheap(&mut a[0..i], 0)
    }
}

#[test]
fn test_heapsort() {
    let mut a = unsorted(3*D+17);
    heapsort(&mut a);
    for i in 1..a.len() {
        assert!(a[i-1] <= a[i]);
    }
}

#[test]
fn bench_heapsort() {
    Criterion::default().bench_function_over_inputs("heapsort",
        |b, &&size| {
            let mut a: Vec<u32> = (0u32..size as u32).collect();
            b.iter(|| heapsort(&mut a))
        }, &[K, 4*K, 16*K, 64*K]);
}

fn extract(a: &mut Vec<u32>) -> u32 {
    assert!(a.len() > 0);
    let r = a[0];
    let m = a.pop().unwrap();
    if a.len() > 0 {
        a[0] = m;
        downheap(a, 0);
    };
    r
}

fn heapsort_extract(mut a: &mut Vec<u32>) -> Vec<u32> {
    heapify(a);
    let mut r = Vec::with_capacity(a.len());
    while a.len() > 0 {
        let v = extract(&mut a);
        r.push(v);
    };
    r.reverse();
    r
}

#[test]
fn test_heapsort_extract() {
    let n = 3*D+17;
    let mut a = unsorted(n);
    let r = heapsort_extract(&mut a);
    assert!(r.len() == n);
    for i in 1..n {
        assert!(r[i-1] <= r[i]);
    }
}

#[test]
fn bench_heapsort_extract() {
    let a: Vec<u32> = (0..N_BENCH).collect();
    Criterion::default().bench_function(
        "heapsort_extract",
        |b| b.iter(|| {
            let r = heapsort_extract(&mut a.clone());
            assert!(r[0] == 0)}));
}

fn upheap(a: &mut[u32], i0: usize) {
    let mut i = i0;
    while i > 0 {
        let p = (i - 1) / D;
        let mut m = p;
        for j in D*p+1..D*p+1+D {
            if j >= a.len() {
                break
            };
            if a[j] > a[m] {
                m = j
            }
        };
        if m == p {
            return
        };
        a.swap(p, m);
        i = p;
    }
}

fn insert(mut a: &mut Vec<u32>, v: u32) {
    let i = a.len();
    a.push(v);
    upheap(&mut a, i)
}

fn heapsort_insert(a: &mut Vec<u32>) {
    let n = a.len();
    let mut t = Vec::with_capacity(n);
    while a.len() > 0 {
        let v = a.pop().unwrap();
        insert(&mut t, v);
    };
    for _ in 0..n {
        let v = extract(&mut t);
        a.push(v);
    };
    a.reverse()
}

#[test]
fn test_heapsort_insert() {
    let n = 3*D+17;
    let mut a = unsorted(n);
    heapsort_insert(&mut a);
    assert!(a.len() == n);
    for i in 1..n {
        assert!(a[i-1] <= a[i]);
    }
}

#[test]
fn bench_heapsort_insert() {
    let mut a: Vec<u32> = (0..N_BENCH).collect();
    Criterion::default().bench_function(
        "heapsort_insert", |b| b.iter(|| heapsort_insert(&mut a)));
}

pub fn main() {
    let a = unsorted(100);
    let mut a_std = a.clone();
    heapsort(&mut a_std);
    let mut a_extract = a.clone();
    let r_extract = heapsort_extract(&mut a_extract);
    assert!(a_std == r_extract);
    let mut a_insert = a.clone();
    heapsort_insert(&mut a_insert);
    assert!(a_std == a_insert);
    for i in 0..a_std.len() {
        println!("{}", a_std[i])
    }
}