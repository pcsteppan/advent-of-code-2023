// the input format is:
// px py pz @ vx vy vz

#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
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
        let filtered_str = str.replace(['@', ','], "");
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

    // referenced implementation from: https://aoc.csokavar.hu/?day=24
    fn find_point_of_intersection(&self, other: &Particle) -> Option<Vec3> {
        let determinant = self.velocity.x * other.velocity.y - self.velocity.y * other.velocity.x;

        // lines are parallel
        if determinant == 0.0 {
            return None;
        }

        let b0 = self.velocity.x * self.position.y - self.velocity.y * self.position.x;
        let b1 = other.velocity.x * other.position.y - other.velocity.y * other.position.x;

        Some(Vec3 {
            x: (other.velocity.x * b0 - self.velocity.x * b1) / determinant,
            y: (other.velocity.y * b0 - self.velocity.y * b1) / determinant,
            z: 0.,
        })
    }

    fn in_future(&self, position: &Vec3) -> bool {
        (position.x - self.position.x).signum() == self.velocity.x.signum()
    }
}

fn find_all_intersections_count(
    particles: &Vec<Particle>,
    lower_bound: f64,
    upper_bound: f64,
) -> usize {
    let mut count = 0;
    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            let intersection_point = particles[i].find_point_of_intersection(&particles[j]);
            if let Some(pt) = intersection_point.clone() {
                if !pt.within_region(lower_bound, upper_bound)
                    || particles[i].position == particles[j].position
                    || !particles[i].in_future(&pt)
                    || !particles[j].in_future(&pt)
                {
                    continue;
                }

                count += 1;
            }
        }
    }

    count
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
    #[test]
    fn test3() {
        let input = include_str!("../input.txt");
        let particles: Vec<Particle> = input.lines().map(Particle::from_str).collect();
        let valid_intersection_point_count =
            find_all_intersections_count(&particles, 200_000_000_000_000., 400_000_000_000_000.);

        assert_eq!(valid_intersection_point_count, 27328);
    }
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
