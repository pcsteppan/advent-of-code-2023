// the input format is:
// px py pz @ vx vy vz

use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn within_region(&self, p1: f64, p2: f64) -> bool {
        self.x >= p1 && self.x <= p2 && self.y >= p1 && self.y <= p2
    }
}

struct Particle {
    position: Vec3,
    velocity: Vec3,
}

impl Particle {
    fn from_str(str: &str) -> Particle {
        let filtered_str = str.replace("@", "").replace(",", "");
        let mut iter = filtered_str.split_whitespace();
        let position = Vec3 {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
            z: iter.next().unwrap().parse().unwrap(),
        };
        let velocity = Vec3 {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
            z: iter.next().unwrap().parse().unwrap(),
        };
        Particle { position, velocity }
    }

    fn find_point_of_intersection(&self, other: &Particle) -> Option<Vec3> {
        let (p1, p2) = (&self.position, self.position.add(&self.velocity));
        let (q1, q2) = (&other.position, other.position.add(&other.velocity));

        let demoninator = (p1.x - p2.x) * (q1.y - q2.y) - (p1.y - p2.y) * (q1.x - q2.x);

        // lines are parallel
        if demoninator < 0. {
            return None;
        }

        let determinant_x = ((p1.x * p2.y - p1.y * p2.x) * (q1.x - q2.x)
            - (p1.x - p2.x) * (q1.x * q2.y - q1.y * q2.x))
            / demoninator;
        let determinant_y = ((p1.x * p2.y - p1.y * p2.x) * (q1.y - q2.y)
            - (p1.y - p2.y) * (q1.x * q2.y - q1.y * q2.x))
            / demoninator;

        Some(Vec3 {
            x: determinant_x,
            y: determinant_y,
            z: 0.,
        })
    }
}

fn find_all_intersections_count(
    particles: &Vec<Particle>,
    lower_bound: f64,
    upper_bound: f64,
) -> usize {
    let mut intersections = vec![];
    for i in 0..particles.len() {
        let mut skip = false;
        for j in i + 1..particles.len() {
            if skip {
                continue;
            }
            let intersection_point = particles[i].find_point_of_intersection(&particles[j]);
            if let Some(pt) = intersection_point.clone() {
                if !pt.within_region(lower_bound, upper_bound)
                    || particles[i].position == particles[j].position
                {
                    continue;
                }

                println!("{:?}, {} {}", pt, i, j);
                intersections.push(j);
                skip = true;
                break;
            }
        }
    }

    println!("{:?}", intersections.clone());
    intersections.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let particles: Vec<Particle> = input.lines().map(Particle::from_str).collect();

    let valid_intersection_point_count =
        find_all_intersections_count(&particles, 200_000_000_000_000., 400_000_000_000_000.);

    println!("part 1: {}", valid_intersection_point_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        // tests particle intersection
        let p1 = Particle {
            position: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            velocity: Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        };
        let p2 = Particle {
            position: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            velocity: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
        };
        let intersection_point = p1.find_point_of_intersection(&p2);
        assert_eq!(
            intersection_point,
            Some(Vec3 {
                x: 0.,
                y: 0.,
                z: 0.
            })
        );
    }

    #[test]
    fn test2() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        let particles: Vec<Particle> = input.lines().map(Particle::from_str).collect();

        let valid_intersection_point_count = find_all_intersections_count(&particles, 7., 24.);
        assert_eq!(valid_intersection_point_count, 2);
    }
}
