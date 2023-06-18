#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, fork_with_priority, get_time, getpid, wait, yield_};

static NUM: usize = 20;
const N: usize = 10;
static P: i32 = 10007;
type Arr = [[i32; N]; N];

fn work(times: isize) {
    let mut a: Arr = Default::default();
    let mut b: Arr = Default::default();
    let mut c: Arr = Default::default();
    for i in 0..N {
        for j in 0..N {
            a[i][j] = 1;
            b[i][j] = 1;
        }
    }
    yield_();
    println!("pid {} is running ({} times)!.", getpid(), times);
    for t in 0..times {
        for i in 0..N {
            for j in 0..N {
                c[i][j] = 0;
                #[allow(clippy::needless_range_loop)]
                for k in 0..N {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % P;
                }
            }
        }
        for i in 0..N {
            for j in 0..N {
                a[i][j] = c[i][j];
                b[i][j] = c[i][j];
            }
        }
        if t % 1000 == 0 {
            println!("pid {} run {} times!.", getpid(), t);
        }
    }
    println!("pid {} done!.", getpid());
    exit(0);
}

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..NUM {
        let current_time = get_time();
        let times = (current_time as i32 as isize) * (current_time as i32 as isize) % 2000;
        let pid = fork_with_priority((times/100+4) as u32);
        if pid == 0 {
            work(times * 10);
        }
    }

    println!("fork ok.");

    let mut exit_code: i32 = 0;
    for _ in 0..NUM {
        if wait(&mut exit_code) < 0 {
            panic!("wait failed.");
        }
    }
    assert!(wait(&mut exit_code) < 0);
    println!("matrix passed.");
    0
}
