mod radix;

use rand::prelude::*;

// Other ones:
// Kirkpatrick–Reisch sort
// Timsort -> underlying implementation for vec.sort in rust

fn main() {
    let mut results = (0i64, 0i64, 0i64);
    const BENCH_COUNT: usize = 100;
    const BENCH_LENGTH: usize = 1000;
    for _ in 0..BENCH_COUNT {
        let (a, b, c) = benchmark(BENCH_LENGTH);
        results.0 += a;
        results.1 += b;
        results.2 += c;
    }
    const BENCH_COUNT_F: f64 = BENCH_COUNT as f64;
    let avg = (results.0 as f64 / BENCH_COUNT_F,
               results.1 as f64 / BENCH_COUNT_F,
               results.2 as f64 / BENCH_COUNT_F);
    println!("Average runtime on vectors of length {} over {} iterations", BENCH_LENGTH, BENCH_COUNT);
    println!("Base 256: {}ns", avg.0);
    println!("  Base 2: {}ns", avg.1);
    println!("std sort: {}ns", avg.2);
}

fn benchmark(test_len: usize) -> (i64, i64, i64) {
    let sorted = |v: &[u64]| {
        v.windows(2).all(|w| w[0] <= w[1])
    };
    let nums = rand_vec(test_len);
    let (speed_base256, base256) = {
        let start = chrono::Utc::now();
        let res = Vec::from(radix::base256(&nums));
        let end = chrono::Utc::now();
        (end.signed_duration_since(start), res)
    };
    let (speed_base2, base2) = {
        let start = chrono::Utc::now();
        let res = Vec::from(radix::base2(&nums));
        let end = chrono::Utc::now();
        (end.signed_duration_since(start), res)
    };
    let (speed_std_sort, std_sort) = {
        let start = chrono::Utc::now();
        let mut res = nums.clone();
        res.sort();
        let end = chrono::Utc::now();
        (end.signed_duration_since(start), res)
    };

    if !sorted(&base256) {
        println!("Base 256 is not sorted!");
        println!("IDX | NUM");
        for (i, num) in base256.iter().enumerate() {
            println!("{:>3} | {:>3}", i, num);
        }
        unreachable!()
    }

    assert!(sorted(&base256));
    assert!(sorted(&base2));
    assert!(sorted(&std_sort));

    (
        speed_base256.num_nanoseconds().unwrap(),
        speed_base2.num_nanoseconds().unwrap(),
        speed_std_sort.num_nanoseconds().unwrap()
    )
}

fn _rand_vec(len: usize) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u64> = (0..len as u64).collect();
    nums.shuffle(&mut rng);
    nums
}

fn rand_vec(len: usize) -> Vec<u64> {
    let mut nums = Vec::with_capacity(len);
    for _ in 0..len {
        nums.push(rand::random());
    }
    nums
}
