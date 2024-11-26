use std::usize;
fn min<N>(a: N, b: N) -> N
where
    N: std::cmp::PartialOrd,
{
    if a >= b {
        b
    } else {
        a
    }
}

pub fn dp_rec_mc(amount: u32) -> u32 {
    // init
    let money_vec: Vec<usize> = vec![1, 2, 5, 10, 20, 30, 50, 100];
    let mut amount_vec = (0..=amount)
        .map(|_| 99999999 as usize)
        .collect::<Vec<usize>>();
    let amount = amount as usize;
    amount_vec[0] = 0;

    // 已有的钱只需要1张
    for &money in money_vec.iter() {
        if money > amount {
            break;
        }
        amount_vec[money] = 1;
    }

    // dp
    'amount: for i in 1..=amount {
        for &money in money_vec.iter() {
            if i < money {
                continue 'amount;
            }
            amount_vec[i] = min(amount_vec[i], amount_vec[i - money] + 1)
        }
    }

    amount_vec[amount] as u32
}
