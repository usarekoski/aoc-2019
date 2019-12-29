use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}

fn parse(map: Vec<String>) -> Vec<Point> {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, position)| match position {
                    '.' => None,
                    '#' => Some(Point {
                        x: x as i64,
                        y: y as i64,
                    }),
                    _ => panic!("unexpected character {}", position),
                })
        })
        .flatten()
        .collect()
}

// Is c between a and b?
fn is_seen(a: Point, b: Point, c: Point) -> bool {
    let abx = b.x - a.x;
    let aby = b.y - a.y;
    let acx = c.x - a.x;
    let acy = c.y - a.y;
    if abx == 0 {
        return acx != 0 || aby / acy < 1;
    }
    if acx == 0 {
        return true;
    }
    if aby == 0 {
        return acy != 0 || abx / acx < 1;
    }
    if acy == 0 {
        return true;
    }
    let same_side = (aby / acy > 0) && (abx / acx > 0);
    let is_closer = abx.pow(2) + aby.pow(2) < acx.pow(2) + acy.pow(2);
    let angle_differs = aby * acx != acy * abx;
    // angle differs, side differs or b is closer than c.
    angle_differs || !same_side || is_closer
}

fn in_sight(asteroids: &Vec<Point>, a: Point, b: Point) -> bool {
    asteroids
        .iter()
        .filter(|&&c| c != a && c != b)
        .all(|c| is_seen(a, b, *c))
}

fn count_between_center(asteroids: &Vec<Point>, p: Point) -> usize {
    let center = Point { x: 0, y: 0 };
    asteroids
        .iter()
        .filter(|&&c| c != p && c != center)
        .filter(|c| !is_seen(center, p, **c))
        .count()
}

fn best_position(asteroids: &Vec<Point>) -> (Point, i64) {
    let asteroids_in_sight = asteroids.iter().map(|&point| {
        let num_seen = asteroids
            .iter()
            .filter(|&&other| other != point && in_sight(&asteroids, point, other))
            .count();
        (point, num_seen as i64)
    });

    // dbg!(&asteroids_in_sight.clone().collect::<Vec<_>>());
    asteroids_in_sight.max_by_key(|(_, n)| *n).unwrap()
}

pub fn solve1(map: Vec<String>) -> i64 {
    let asteroids = parse(map);
    let (_point, num_seen) = best_position(&asteroids);
    num_seen
}

pub fn solve2(map: Vec<String>) -> i64 {
    let asteroids = parse(map);
    let (center, _) = best_position(&asteroids);
    let asteroids_centered: Vec<Point> = asteroids
        .iter()
        .filter(|p| **p != center)
        .map(|p| Point {
            x: p.x - center.x,
            y: -(p.y - center.y), // Flip y-axis to right side up.
        })
        .collect();
    let mut point_order: Vec<(usize, f64, Point)> = asteroids_centered
        .iter()
        .map(|p| {
            let round = count_between_center(&asteroids_centered, *p);
            // Clockwise positive angle between p and positive y-axis.
            let angle = -(p.x as f64).atan2(-p.y as f64) + PI;
            (round, angle, *p)
        })
        .collect();

    // dbg!(&point_order);

    point_order.sort_by(|a, b| {
        if a.0 == b.0 {
            return a.1.partial_cmp(&b.1).unwrap();
        }
        a.0.cmp(&b.0)
    });

    let (_, _, p200) = point_order[200 - 1];
    let orig_p200 = Point {
        x: p200.x + center.x,
        y: -p200.y + center.y,
    };
    dbg!(&orig_p200);

    orig_p200.x * 100 + orig_p200.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = ".#..#
.....
#####
....#
...##"
            .lines()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve1(input), 8);
    }

    #[test]
    fn test2_1() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            .lines()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve2(input), 802);
    }
}
