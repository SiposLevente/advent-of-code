use point::point::Point;
use std::cmp::Ordering;

pub struct Line {
    p1: Point<isize>,
    p2: Point<isize>,
}

impl Line {
    pub fn new(p1: Point<isize>, p2: Point<isize>) -> Line {
        Line { p1, p2 }
    }

    pub fn new_from_coords(p1x: isize, p1y: isize, p2x: isize, p2y: isize) -> Line {
        let p1 = Point::new(p1x, p1y);
        let p2 = Point::new(p2x, p2y);
        Line { p1, p2 }
    }

    pub fn line_cords(&mut self) -> Vec<Point<isize>> {
        let mut points: Vec<Point<isize>> = vec![];
        let p = Point::new(self.p1.x, self.p1.y);
        points.push(p);

        while self.p1 != self.p2 {
            match self.p1.x.cmp(&self.p2.x) {
                Ordering::Less => self.p1.offset(1, 0),
                Ordering::Equal => {}
                Ordering::Greater => self.p1.offset(-1, 0),
            }

            match self.p1.y.cmp(&self.p2.y) {
                Ordering::Less => self.p1.offset(0, 1),
                Ordering::Equal => {}
                Ordering::Greater => self.p1.offset(0, -1),
            }

            let p = Point::new(self.p1.x, self.p1.y);
            points.push(p);
        }

        points
    }
}
