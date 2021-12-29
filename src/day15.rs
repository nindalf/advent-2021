use std::collections::{BinaryHeap, HashMap};

use crate::matrix::{Matrix, Point};

impl Matrix {
    #[allow(dead_code)]
    fn dijkstra_cost(&self, start: &Point, end: &Point) -> u32 {
        let mut heap = BinaryHeap::new();
        let start = State::new(*start, 0);
        let mut visited = HashMap::new();
        heap.push(start);

        while let Some(state) = heap.pop() {
            if state.point == *end {
                return state.cost;
            }
            if visited.contains_key(&state.point) {
                continue;
            }
            visited.insert(state.point, state.cost);
            for neighbour in self.neighbours(state.point) {
                if let Some(value) = self.value(&neighbour) {
                    let cost_to_neighbour = state.cost + *value;
                    // Already a better route to this point
                    if let Some(previous_cost) = visited.get(&neighbour) {
                        if *previous_cost < cost_to_neighbour {
                            continue;
                        }
                    }
                    heap.push(State {
                        point: neighbour,
                        cost: cost_to_neighbour,
                    })
                }
            }
        }
        unreachable!("Must find the end")
    }
}

// Copied from the example on https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    point: Point,
    cost: u32,
}

impl State {
    fn new(point: Point, cost: u32) -> State {
        State { point, cost }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that the we flip the ordering on costs.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Point;
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day15-test.txt", false, 40)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day15.txt", false, 696)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day15-test.txt", true, 315)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day15.txt", true, 2952)
    }

    fn test(test_file: &str, construct_extended: bool, expected: u32) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let matrix = if construct_extended {
            super::Matrix::new(&input)?.construct_extended_matrix()
        } else {
            super::Matrix::new(&input)?
        };
        let start = Point { x: 0, y: 0 };
        let end = Point {
            x: matrix.max_x - 1,
            y: matrix.max_y - 1,
        };
        assert_eq!(matrix.dijkstra_cost(&start, &end), expected);
        Ok(())
    }
}
