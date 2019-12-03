use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub struct Step {
    dir: Dir,
    dist: i32,
}

impl FromStr for Step {
    type Err = Box<dyn ::std::error::Error>;

    // R30
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);
        let dir = match dir {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("unexpected dir"),
        };

        Ok(Step {
            dir: dir,
            dist: dist.parse()?,
        })
    }
}

fn manhattan_distance((x_1, y_1): (i32, i32), (x_2, y_2): (i32, i32)) -> i32 {
    (x_1 - x_2).abs() + (y_1 - y_2).abs()
}

pub fn solve1(paths: Vec<String>) -> i32 {
    let paths: Vec<Vec<Step>> = paths
        .iter()
        .map(|path| path.split(',').map(|step| step.parse().unwrap()).collect())
        .collect();

    let paths_points: Vec<HashSet<(i32, i32)>> = paths
        .iter()
        .map(|path| {
            let mut points: HashSet<(i32, i32)> = HashSet::new();
            let mut x = 0;
            let mut y = 0;
            for step in path.iter() {
                match step.dir {
                    Dir::Right => {
                        for x_d in (x + 1)..=(x + step.dist) {
                            x = x_d;
                            points.insert((x, y));
                        }
                    }
                    Dir::Left => {
                        for x_d in ((x - step.dist)..x).rev() {
                            x = x_d;
                            points.insert((x, y));
                        }
                    }
                    Dir::Up => {
                        for y_d in (y + 1)..=(y + step.dist) {
                            y = y_d;
                            points.insert((x, y));
                        }
                    }
                    Dir::Down => {
                        for y_d in ((y - step.dist)..y).rev() {
                            y = y_d;
                            points.insert((x, y));
                        }
                    }
                }
            }
            points
        })
        .collect();

    let min_dist = paths_points[0]
        .intersection(&paths_points[1])
        .filter(|&&p| p != (0, 0))
        .map(|&p| manhattan_distance((0, 0), p))
        .min();

    min_dist.unwrap()
}

pub fn solve2(paths: Vec<String>) -> i32 {
    let paths: Vec<Vec<Step>> = paths
        .iter()
        .map(|path| path.split(',').map(|step| step.parse().unwrap()).collect())
        .collect();

    let paths_points: Vec<HashMap<(i32, i32), i32>> = paths
        .iter()
        .map(|path| {
            // points with distances
            let mut points: HashMap<(i32, i32), i32> = HashMap::new();
            let mut x = 0;
            let mut y = 0;
            let mut distance = 0;
            for step in path.iter() {
                match step.dir {
                    Dir::Right => {
                        for x_d in (x + 1)..=(x + step.dist) {
                            x = x_d;
                            distance += 1;
                            if points.get(&(x, y)).is_none() {
                                points.insert((x, y), distance);
                            }
                        }
                    }
                    Dir::Left => {
                        for x_d in ((x - step.dist)..x).rev() {
                            x = x_d;
                            distance += 1;
                            if points.get(&(x, y)).is_none() {
                                points.insert((x, y), distance);
                            }
                        }
                    }
                    Dir::Up => {
                        for y_d in (y + 1)..=(y + step.dist) {
                            y = y_d;
                            distance += 1;
                            if points.get(&(x, y)).is_none() {
                                points.insert((x, y), distance);
                            }
                        }
                    }
                    Dir::Down => {
                        for y_d in ((y - step.dist)..y).rev() {
                            y = y_d;
                            distance += 1;
                            if points.get(&(x, y)).is_none() {
                                points.insert((x, y), distance);
                            }
                        }
                    }
                }
            }
            points
        })
        .collect();

    let second_path = &paths_points[1];
    let min_dist = paths_points[0]
        .iter()
        .filter(|&(k, _)| *k != (0, 0))
        .filter_map(|(k, d1)| second_path.get(k).map(|d2| d2 + d1))
        .min();

    min_dist.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_1() {
        let paths = vec!["R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()];
        assert_eq!(solve1(paths), 6);
    }

    #[test]
    fn test1_2() {
        let paths = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
            "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
        ];
        assert_eq!(solve1(paths), 159);
    }

    #[test]
    fn test1_3() {
        let paths = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
        ];
        assert_eq!(solve1(paths), 135);
    }

    #[test]
    fn test2_1() {
        let paths = vec!["R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()];
        assert_eq!(solve2(paths), 30);
    }

    #[test]
    fn test2_2() {
        let paths = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
            "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
        ];
        assert_eq!(solve2(paths), 610);
    }

    #[test]
    fn test2_3() {
        let paths = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
        ];
        assert_eq!(solve2(paths), 410);
    }
}
