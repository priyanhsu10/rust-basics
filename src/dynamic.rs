use std::{collections::HashMap, hash::Hash, mem};
pub fn exp() {
    // fib_exp();
    // grid_traversar_exp();
    can_sum_exp();
}

fn can_sum_exp() {
    let a = [1, 2, 3, 4, 5, 7];
    println!("{}", can_sum(7, &a))
}
fn grid_traversar_exp() {
    // let mut r = grid_traverser(2, 3);
    // println!("{}", r);
    // r = grid_traverser(3, 3);
    // println!("{}", r);
    // r = grid_traverser(18, 18);
    // println!("{}", r);
    //memo exmaple
    let mut memo: HashMap<(u8, u8), u64> = HashMap::new();
    let mut r = grid_traverser_memo(2, 3, &mut memo);
    println!("{}", r);
    r = grid_traverser_memo(3, 3, &mut memo);
    println!("{}", r);
    r = grid_traverser_memo(18, 18, &mut memo);
    println!("{}", r);
}

fn can_sum(target: u16, nums: &[u16]) -> bool {
    for x in nums {
        if target - x == 0 {
            return true;
        }
    }

    false
}
fn grid_traverser(m: u8, n: u8) -> u64 {
    if m == 1 && n == 1 {
        return 1;
    }
    if m == 0 || n == 0 {
        return 0;
    }

    return grid_traverser(m - 1, n) + grid_traverser(m, n - 1);
}
fn grid_traverser_memo(m: u8, n: u8, memo: &mut HashMap<(u8, u8), u64>) -> u64 {
    if m == 1 && n == 1 {
        return 1;
    }
    if m == 0 || n == 0 {
        return 0;
    }
    let key1 = (m, n);
    let key2 = (n, m);
    if memo.contains_key(&key1) || memo.contains_key(&key2) {
        return memo[&key1];
    }

    let r = grid_traverser_memo(m - 1, n, memo) + grid_traverser_memo(m, n - 1, memo);

    memo.insert(key1, r);
    memo.insert(key2, r);
    return r;
}

fn fib_exp() {
    let mut momo: HashMap<u32, u64> = HashMap::new();
    //fibonanci problem  solving using momoizatoin
    let r = fib(50, &mut momo);
    println!("{}", r);
}

pub fn fib(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if memo.contains_key(&n) {
        return memo[&n];
    }
    if n <= 2 {
        return 1;
    }
    let r = fib(n - 1, memo) + fib(n - 2, memo);
    memo.insert(n, r);
    return memo[&n];
}
