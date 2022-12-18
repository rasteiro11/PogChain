#[derive(Debug)]
struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

struct ECC {
    pub a: f64,
    pub b: f64,
}

impl ECC {
    pub fn new(a: f64, b: f64) -> Self {
        ECC { a, b }
    }

    fn double_and_add(&self, n: i64, p: &Point) -> Point {
        let mut temp = Point::new(p.x, p.y);

        let n_binary = format!("{:b}", n);

        for c in n_binary.chars().skip(1) {
            let actual_bit = c.to_digit(10).expect("Could not parse char to digit");

            temp = self.point_addition(&temp, &temp);

            if actual_bit == 1 {
                temp = self.point_addition(&temp, &p);
            }
        }
        temp
    }

    fn point_addition(&self, p: &Point, q: &Point) -> Point {
        let x1 = p.x;
        let y1 = p.y;
        let x2 = q.x;
        let y2 = q.y;

        let m;

        if x1 == x2 && y1 == y2 {
            m = (3 as f64 * x1 * x1 + self.a) / (2 as f64 * y1);
        } else {
            m = (y2 - y1) / (x2 - x1);
        }

        let x3 = m * m - x2 - x1;
        let y3 = m * (x1 - x3) - y1;

        Point::new(x3, y3)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::{Point, ECC};

    #[test]
    fn test_ecc() {
        let ecc = ECC::new(0.0, 7.0);
        let generator = Point::new(1.0, 1.0);

        let test = ecc.point_addition(&generator, &generator);
        assert_eq!(test.x, 0.25);
        assert_eq!(test.y, 0.125);

        let test2 = ecc.double_and_add(100, &generator);
        assert_eq!(test2.x, 0.0001000004649412779);
        assert_eq!(test2.y, 9.999907011692133e-7);
    }

    #[test]
    fn test_encryption() {
        let ecc = ECC::new(3.0, 7.0);
        let generator = Point::new(-2.0, 1.0);
        let mut rng = rand::thread_rng();

        let a = rng.gen_range(0..1000);
        let b = rng.gen_range(0..1000);

        let alicePublicKey = ecc.double_and_add(a, &generator);
        let bobPublicKey = ecc.double_and_add(b, &generator);

        let aliceSecretKey = ecc.double_and_add(a, &bobPublicKey);
        let bobSecretKey = ecc.double_and_add(b, &alicePublicKey);
    }
}
