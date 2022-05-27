fn sum(m: i32) -> i64 {
    ((m * (m + 1)) / 2) as i64
}
fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let sum = sum(m);
    let mut rs = Vec::new();
    let begin = (sum - m as i64) / (m as i64 + 1);
    for i in begin..m as i64 {
        if (sum - i) % (i + 1) == 0 {
            let b = (sum - i) / (i + 1);
            if b < m as i64 {
                rs.push((i as i32, b as i32));
            }
        }
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;
    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(remove_nb(n), exp)
    }
    #[test]
    fn basics_remove_nb() {
        testing(26, vec![(15, 21), (21, 15)]);
        testing(100, vec![]);
        testing(101, vec![(55, 91), (91, 55)]);
        testing(102, vec![(70, 73), (73, 70)]);
    }
}
