pub fn greedy_coin_change(amount: u32) -> Vec<u32> {
    let mut coins = vec![1, 5, 10, 25];
    coins.sort();
    coins.reverse();

    let mut change = vec![];
    let mut remaining = amount;

    for coin in coins {
        while remaining >= coin {
            remaining -= coin;
            change.push(coin);
        }
    }
    change
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greedy_coin_change() {
        assert_eq!(greedy_coin_change(10), vec![10]);
        assert_eq!(greedy_coin_change(11), vec![10, 1]);
        assert_eq!(greedy_coin_change(0), vec![]);
        assert_eq!(greedy_coin_change(1), vec![1]);
        assert_eq!(greedy_coin_change(100), vec![25, 25, 25, 25]);
        assert_eq!(greedy_coin_change(29), vec![25, 1, 1, 1, 1]);
        assert_eq!(greedy_coin_change(37), vec![25, 10, 1, 1]);
    }
}
