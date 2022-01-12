use std::ops::Sub;

struct Cuboid {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
    state: State,
}

enum State {
    On,
    Off,
}

struct Range(i32, i32);

impl Sub for Range {
    type Output = Range;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Cuboid {
    fn intersect(self, other: &Cuboid) -> (Cuboid, Option<Cuboid>, Option<Cuboid>) {
        let no_x_overlap =
            Cuboid::no_overlap((self.x_start, self.x_end), (other.x_start, other.x_end));
        let no_y_overlap =
            Cuboid::no_overlap((self.y_start, self.y_end), (other.y_start, other.y_end));
        let no_z_overlap =
            Cuboid::no_overlap((self.z_start, self.z_end), (other.z_start, other.z_end));
        // if exactly 2 are true, there is
        if (no_x_overlap && no_y_overlap)
            || (no_x_overlap && no_z_overlap)
            || (no_y_overlap && no_z_overlap)
        {
            return (self, None, None);
        }
        (self, None, None)
    }

    fn no_overlap(l1: (i32, i32), l2: (i32, i32)) -> bool {
        if (l2.0 < l1.0 && l1.0 < l2.1) || (l2.0 < l1.1 && l1.1 < l2.1) {
            return false;
        }
        if (l1.0 < l2.0 && l2.0 < l1.1) || (l1.0 < l2.1 && l2.1 < l1.1) {
            return false;
        }
        true
    }
}
