fn fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

pub fn solve1(masses: Vec<i64>) -> i64 {
    masses.iter().copied().map(fuel).sum()
}

pub fn solve2(masses: Vec<i64>) -> i64 {
    let mut sum = 0;
    for &mass in masses.iter() {
        let mut fuel_base = fuel(mass);
        let mut fuel_added = fuel(fuel_base);
        while fuel_added > 0 {
            fuel_base += fuel_added;
            fuel_added = fuel(fuel_added);
        }
        sum += fuel_base;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve1(vec![12]), 2);
        assert_eq!(solve1(vec![14]), 2);
        assert_eq!(solve1(vec![1969]), 654);
        assert_eq!(solve1(vec![100756]), 33583);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(vec![14]), 2);
        assert_eq!(solve2(vec![1969]), 966);
        assert_eq!(solve2(vec![100756]), 50346);
    }
}
