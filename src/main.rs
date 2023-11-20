use rand::prelude::*;

struct Point {
    x: f64,
    y: f64,
}

struct PointGenerator {
    rng: ThreadRng,
}

impl PointGenerator {
    fn new() -> PointGenerator {
        PointGenerator {
            rng: rand::thread_rng(),
        }
    }

    fn gen(&mut self) -> Point {
        Point {
            x: self.rng.gen(),
            y: self.rng.gen(),
        }
    }
}

type Predicate = fn(&Point) -> bool;

fn is_in_unit_circle_fn(p: &Point) -> bool {
    p.x * p.x + p.y * p.y < 1.0
}

fn integrate(pred: Predicate, count: usize) -> f64 {
    let mut pg = PointGenerator::new();

    (0..count).map(|_| pg.gen()).filter(|p| pred(p)).count() as f64 / (count as f64)
}

fn main() {
    let pi = integrate(is_in_unit_circle_fn, 100_000_000) * 4.0;
    println!("pi: {}", pi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();

        // accumulate 10000 random numbers,
        // check if they are in the range [0, 1),
        // and check if their mean is close to 0.5

        let mut sum = 0.0;
        let count = 10000;
        let fwhm = 65.0;
        for _ in 0..count {
            let x: f64 = rng.gen();
            assert!(x >= 0.0 && x < 1.0);
            sum += x;
        }

        let expected_mean: f64 = 0.5 * (count as f64);
        assert!((sum - expected_mean).abs() < fwhm);
    }

    #[test]
    fn test_pi() {
        let pi = integrate(is_in_unit_circle_fn, 10_000) * 4.0;
        assert!((pi - std::f64::consts::PI).abs() < 0.5);
    }
}
