use std::ops::{Add, Index, Mul, MulAssign, Neg, Sub};

pub type Coord = i32;

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct Vector3 {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

impl Vector3 {
    pub fn manhattan_distance(self) -> Coord {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl From<String> for Vector3 {
    fn from(s: String) -> Self {
        let mut output = Vector3::default();
        let mut split = s.split(',');
        output.x = split.next().unwrap().parse().unwrap();
        output.y = split.next().unwrap().parse().unwrap();
        output.z = split.next().unwrap().parse().unwrap();
        output
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Matrix4([[Coord; 4]; 4]);

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4([
            [
                self[0][0] * rhs[0][0]
                    + self[0][1] * rhs[1][0]
                    + self[0][2] * rhs[2][0]
                    + self[0][3] * rhs[3][0],
                self[0][0] * rhs[0][1]
                    + self[0][1] * rhs[1][1]
                    + self[0][2] * rhs[2][1]
                    + self[0][3] * rhs[3][1],
                self[0][0] * rhs[0][2]
                    + self[0][1] * rhs[1][2]
                    + self[0][2] * rhs[2][2]
                    + self[0][3] * rhs[3][2],
                self[0][0] * rhs[0][3]
                    + self[0][1] * rhs[1][3]
                    + self[0][2] * rhs[2][3]
                    + self[0][3] * rhs[3][3],
            ],
            [
                self[1][0] * rhs[0][0]
                    + self[1][1] * rhs[1][0]
                    + self[1][2] * rhs[2][0]
                    + self[1][3] * rhs[3][0],
                self[1][0] * rhs[0][1]
                    + self[1][1] * rhs[1][1]
                    + self[1][2] * rhs[2][1]
                    + self[1][3] * rhs[3][1],
                self[1][0] * rhs[0][2]
                    + self[1][1] * rhs[1][2]
                    + self[1][2] * rhs[2][2]
                    + self[1][3] * rhs[3][2],
                self[1][0] * rhs[0][3]
                    + self[1][1] * rhs[1][3]
                    + self[1][2] * rhs[2][3]
                    + self[1][3] * rhs[3][3],
            ],
            [
                self[2][0] * rhs[0][0]
                    + self[2][1] * rhs[1][0]
                    + self[2][2] * rhs[2][0]
                    + self[2][3] * rhs[3][0],
                self[2][0] * rhs[0][1]
                    + self[2][1] * rhs[1][1]
                    + self[2][2] * rhs[2][1]
                    + self[2][3] * rhs[3][1],
                self[2][0] * rhs[0][2]
                    + self[2][1] * rhs[1][2]
                    + self[2][2] * rhs[2][2]
                    + self[2][3] * rhs[3][2],
                self[2][0] * rhs[0][3]
                    + self[2][1] * rhs[1][3]
                    + self[2][2] * rhs[2][3]
                    + self[2][3] * rhs[3][3],
            ],
            [
                self[3][0] * rhs[0][0]
                    + self[3][1] * rhs[1][0]
                    + self[3][2] * rhs[2][0]
                    + self[3][3] * rhs[3][0],
                self[3][0] * rhs[0][1]
                    + self[3][1] * rhs[1][1]
                    + self[3][2] * rhs[2][1]
                    + self[3][3] * rhs[3][1],
                self[3][0] * rhs[0][2]
                    + self[3][1] * rhs[1][2]
                    + self[3][2] * rhs[2][2]
                    + self[3][3] * rhs[3][2],
                self[3][0] * rhs[0][3]
                    + self[3][1] * rhs[1][3]
                    + self[3][2] * rhs[2][3]
                    + self[3][3] * rhs[3][3],
            ],
        ])
    }
}

impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3],
            y: self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3],
            z: self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3],
        }
    }
}

impl Add<Vector3> for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Matrix4([
            [self[0][0], self[0][1], self[0][2], self[0][3] + rhs.x],
            [self[1][0], self[1][1], self[1][2], self[1][3] + rhs.y],
            [self[2][0], self[2][1], self[2][2], self[2][3] + rhs.z],
            [self[3][0], self[3][1], self[3][2], self[3][3]],
        ])
    }
}

impl Index<usize> for Matrix4 {
    type Output = [Coord; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);

    pub const ROTATE_X_90: Matrix4 =
        Matrix4([[1, 0, 0, 0], [0, 0, -1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]);

    pub const ROTATE_Y_90: Matrix4 =
        Matrix4([[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]]);

    pub const ROTATE_Z_90: Matrix4 =
        Matrix4([[0, -1, 0, 0], [1, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);

    pub fn translation(self) -> Vector3 {
        Vector3 {
            x: self[0][3],
            y: self[1][3],
            z: self[2][3],
        }
    }
}
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_ORIENTATIONS: Vec<Matrix4> = {
        let rotate_x_180 = Matrix4::ROTATE_X_90 * Matrix4::ROTATE_X_90;
        let rotate_x_270 = rotate_x_180 * Matrix4::ROTATE_X_90;
        let rotate_y_180 = Matrix4::ROTATE_Y_90 * Matrix4::ROTATE_Y_90;
        let rotate_y_270 = rotate_y_180 * Matrix4::ROTATE_Y_90;
        let rotate_z_180 = Matrix4::ROTATE_Z_90 * Matrix4::ROTATE_Z_90;
        let rotate_z_270 = rotate_z_180 * Matrix4::ROTATE_Z_90;

        // All 24 orientations, using +x as default face.
        vec![
            // +x
            Matrix4::IDENTITY,
            Matrix4::ROTATE_X_90,
            rotate_x_180,
            rotate_x_270,
            // -x
            rotate_z_180,
            rotate_z_180 * Matrix4::ROTATE_X_90,
            rotate_z_180 * rotate_x_180,
            rotate_z_180 * rotate_x_270,
            // +y
            rotate_z_270,
            rotate_z_270 * Matrix4::ROTATE_Y_90,
            rotate_z_270 * rotate_y_180,
            rotate_z_270 * rotate_y_270,
            // -y
            Matrix4::ROTATE_Z_90,
            Matrix4::ROTATE_Z_90 * Matrix4::ROTATE_Y_90,
            Matrix4::ROTATE_Z_90 * rotate_y_180,
            Matrix4::ROTATE_Z_90 * rotate_y_270,
            // +z
            Matrix4::ROTATE_Y_90,
            Matrix4::ROTATE_Y_90 * Matrix4::ROTATE_Z_90,
            Matrix4::ROTATE_Y_90 * rotate_z_180,
            Matrix4::ROTATE_Y_90 * rotate_z_270,
            // -z
            rotate_y_270,
            rotate_y_270 * Matrix4::ROTATE_Z_90,
            rotate_y_270 * rotate_z_180,
            rotate_y_270 * rotate_z_270,
        ]
    };
}

use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

#[allow(dead_code)]
fn part1<R: BufRead>(reader: R) -> String {
    let regions = read_regions(reader);
    let region_transforms = get_region_transforms(&regions);
    count_beacons(regions, region_transforms).to_string()
}

#[allow(dead_code)]
fn part2<R: BufRead>(reader: R) -> String {
    let regions = read_regions(reader);
    let region_transforms = get_region_transforms(&regions);
    get_largest_manhattan_distance_between_scanners(region_transforms).to_string()
}

fn get_region_transforms(regions: &[Region]) -> Vec<Matrix4> {
    let mut region_transforms = vec![Matrix4::IDENTITY; regions.len()];
    let mut visited = vec![false; regions.len()];
    let mut queue = vec![(0, Matrix4::IDENTITY)];

    while let Some((base_index, base_transform)) = queue.pop() {
        visited[base_index] = true;
        region_transforms[base_index] = base_transform;

        for i in 0..regions.len() {
            if i != base_index && !visited[i] {
                if let Some(transform) = regions[i].get_transform_relative_to(&regions[base_index])
                {
                    queue.push((i, base_transform * transform));
                }
            }
        }
    }

    region_transforms
}

fn count_beacons(regions: Vec<Region>, region_transforms: Vec<Matrix4>) -> usize {
    regions
        .into_iter()
        .enumerate()
        .map(|(i, region)| region * region_transforms[i])
        .flat_map(|r| r.beacons)
        .collect::<HashSet<Vector3>>()
        .len()
}

fn get_largest_manhattan_distance_between_scanners(region_transforms: Vec<Matrix4>) -> Coord {
    let translations = region_transforms
        .into_iter()
        .map(Matrix4::translation)
        .collect::<Vec<Vector3>>();

    translations
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| {
            translations[i + 1..]
                .iter()
                .map(move |&b| b - a)
                .map(Vector3::manhattan_distance)
        })
        .max()
        .unwrap()
}

fn read_regions<R: BufRead>(reader: R) -> Vec<Region> {
    let mut regions = Vec::new();
    let lines = &mut reader.lines();
    while let Some(region) = next_region(lines) {
        regions.push(region);
    }
    regions
}

fn next_region<R: BufRead>(lines: &mut Lines<R>) -> Option<Region> {
    lines.next().map(|_| {
        lines
            .map(Result::unwrap)
            .take_while(|line| !line.is_empty())
            .map(Vector3::from)
            .collect::<Region>()
    })
}

#[derive(Default)]
struct Region {
    // The beacon positions:
    beacons: Vec<Vector3>,
    // Markers for finding overlapping regions (distances between every combination of 2 beacons):
    distance_markers: HashSet<Coord>,
    // Markers for orienting regions (vectors for every combination of 2 beacons):
    orientation_markers: HashMap<Vector3, Vector3>,
}

impl Region {
    const REQUIRED_MARKERS_FOR_OVERLAP: usize = 66; // nCr(12, 2)

    fn insert(&mut self, beacon: Vector3) {
        for cur_beacon in &self.beacons {
            let orientation_marker = *cur_beacon - beacon;
            self.orientation_markers
                .insert(orientation_marker, *cur_beacon);
            self.orientation_markers.insert(-orientation_marker, beacon);
            self.distance_markers
                .insert(orientation_marker.manhattan_distance());
        }
        self.beacons.push(beacon);
    }

    fn get_transform_relative_to(&self, other: &Self) -> Option<Matrix4> {
        if self
            .distance_markers
            .intersection(&other.distance_markers)
            .count()
            < Self::REQUIRED_MARKERS_FOR_OVERLAP
        {
            None
        } else {
            for &try_rotation in ALL_ORIENTATIONS.iter() {
                let overlapping_orientation_markers = self
                    .orientation_markers
                    .iter()
                    .map(|(&k, &v)| (try_rotation * k, try_rotation * v))
                    .filter(|(k, _)| other.orientation_markers.contains_key(k))
                    .collect::<HashMap<Vector3, Vector3>>();

                if overlapping_orientation_markers.len() >= Self::REQUIRED_MARKERS_FOR_OVERLAP {
                    let translation = overlapping_orientation_markers
                        .into_iter()
                        .next()
                        .map(|(k, v)| other.orientation_markers[&k] - v)
                        .unwrap();

                    return Some(try_rotation + translation);
                }
            }

            unreachable!()
        }
    }
}

impl Mul<Matrix4> for Region {
    type Output = Self;

    fn mul(self, rhs: Matrix4) -> Self {
        Region {
            beacons: self.beacons.iter().map(|&b| rhs * b).collect(),
            distance_markers: self.distance_markers,
            orientation_markers: self
                .orientation_markers
                .iter()
                .map(|(&k, &v)| (rhs * k, rhs * v))
                .collect(),
        }
    }
}

impl FromIterator<Vector3> for Region {
    fn from_iter<T: IntoIterator<Item = Vector3>>(iter: T) -> Self {
        let mut region = Region::default();
        iter.into_iter().for_each(|v| region.insert(v));
        region
    }
}
mod tests {
    #[test]
    fn test_part1() -> anyhow::Result<()> {
        assert_eq!(
            super::part1(crate::files::buf_reader("inputs/day19.txt")?),
            "330"
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        assert_eq!(
            super::part2(crate::files::buf_reader("inputs/day19.txt")?),
            "9634"
        );
        Ok(())
    }
}
