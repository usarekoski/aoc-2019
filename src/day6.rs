use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};

fn parse(orbits: &Vec<String>) -> Vec<(&str, &str)> {
    orbits
        .iter()
        .map(|orbit| {
            let mut split = orbit.split(')');
            let up = split.next().unwrap();
            let down = split.next().unwrap();
            (up, down)
        })
        .collect()
}

pub fn solve1(orbits: Vec<String>) -> i64 {
    let nodes = parse(&orbits);

    let mut num_orbits = 0;
    let mut queue: VecDeque<(&str, i64)> = VecDeque::new();
    queue.push_back(("COM", 0));
    while !queue.is_empty() {
        let (cur, depth) = queue.pop_front().unwrap();
        num_orbits += depth;
        for (_, child) in nodes.iter().filter(|(s, _)| *s == cur) {
            queue.push_back((child, depth + 1));
        }
    }

    num_orbits
}

pub fn solve2(orbits: Vec<String>) -> i64 {
    let nodes = parse(&orbits);

    let mut visited: HashMap<&str, i64> = HashMap::new();
    let mut unvisited: BinaryHeap<Reverse<(i64, &str)>> = BinaryHeap::new();
    unvisited.push(Reverse((0, "YOU")));
    let target = "SAN";

    while let Some(n) = unvisited.pop() {
        let (d, next) = n.0;
        if visited.contains_key(&next) {
            continue;
        }

        // Down
        for (_, child) in nodes.iter().filter(|(parent, _)| *parent == next) {
            dbg!(child);
            unvisited.push(Reverse((d + 1, child)));
        }
        // Up
        for (parent, _) in nodes.iter().filter(|(_, child)| *child == next) {
            dbg!(parent);
            unvisited.push(Reverse((d + 1, parent)));
        }

        visited.insert(next, d);

        if next == target {
            break;
        }
    }

    let distance = *visited.get(target).unwrap();

    // Count only objects
    distance - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = [
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|&s| String::from(s))
        .collect();
        assert_eq!(solve1(input), 42);
    }

    #[test]
    fn test2() {
        let input = [
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]
        .iter()
        .map(|&s| String::from(s))
        .collect();
        assert_eq!(solve2(input), 4);
    }
}
