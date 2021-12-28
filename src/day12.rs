use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Write,
};

use anyhow::{anyhow, Result};

struct Graph {
    nodes: HashMap<Node, HashSet<Node>>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Node(String);

#[derive(PartialEq, Eq, Clone)]
struct Path(Vec<Node>);

impl Graph {
    #[allow(dead_code)]
    fn new(input: &[String]) -> Result<Graph> {
        let mut graph = Graph {
            nodes: HashMap::new(),
        };
        input
            .iter()
            .map(|connection| graph.add_connection(connection))
            .collect::<Result<Vec<_>>>()?;

        Ok(graph)
    }

    fn add_connection(&mut self, connection: &str) -> Result<()> {
        let mut parts = connection.split('-');
        let one = Node(
            parts
                .next()
                .ok_or(anyhow!("Invalid connection"))?
                .to_owned(),
        );
        let two = Node(
            parts
                .next()
                .ok_or(anyhow!("Invalid connection"))?
                .to_owned(),
        );
        self.nodes
            .entry(one.clone())
            .or_insert_with(HashSet::new)
            .insert(two.clone());
        self.nodes
            .entry(two)
            .or_insert_with(HashSet::new)
            .insert(one);
        Ok(())
    }

    fn get_small_connections(&self, node: &Node) -> impl Iterator<Item = &Node> {
        self.nodes
            .get(node)
            .unwrap()
            .iter()
            .filter(|node| node.is_small())
    }

    fn get_large_connections(&self, node: &Node) -> impl Iterator<Item = &Node> {
        self.nodes
            .get(node)
            .unwrap()
            .iter()
            .filter(|node| !node.is_small())
    }

    #[allow(dead_code)]
    fn part_one(&self) -> usize {
        let skip_small_connection = |path: &Path, node: &Node| path.contains_node(node);
        self.num_possible_paths(&skip_small_connection)
    }

    #[allow(dead_code)]
    fn part_two(&self) -> usize {
        let skip_small_connection = |path: &Path, node: &Node| {
            (path.contains_any_double_small_node() && path.contains_node(node)) || node.0 == "start"
        };
        self.num_possible_paths(&skip_small_connection)
    }

    fn num_possible_paths(&self, skip_small_connection: &dyn Fn(&Path, &Node) -> bool) -> usize {
        let mut start = Path::new();
        start.add_node(&Node("start".to_owned()));

        let end = Node("end".to_owned());

        let mut result = HashSet::new();
        let mut q: VecDeque<Path> = VecDeque::new();
        q.push_back(start);
        while let Some(path) = q.pop_front() {
            let last_node = path.last_node();
            // path is complete
            if last_node == &end {
                result.insert(path.to_string());
                continue;
            }

            for connection in self.get_large_connections(last_node) {
                let mut new_path = path.clone();
                new_path.add_node(connection);
                q.push_back(new_path);
            }

            for connection in self.get_small_connections(last_node) {
                if skip_small_connection(&path, connection) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.add_node(connection);
                q.push_back(new_path);
            }
        }
        result.len()
    }
}

impl Node {
    fn is_small(&self) -> bool {
        self.0.chars().all(|c| c.is_ascii_lowercase())
    }
}

impl Path {
    fn new() -> Path {
        Path(Vec::new())
    }

    fn last_node(&self) -> &Node {
        &self.0[self.0.len() - 1]
    }

    fn add_node(&mut self, node: &Node) {
        self.0.push(node.clone())
    }

    // O(N) for now. Can optimise if necessary.
    fn contains_node(&self, node: &Node) -> bool {
        self.0.iter().any(|n| n == node)
    }

    fn contains_any_double_small_node(&self) -> bool {
        let small_nodes = self
            .0
            .iter()
            .filter(|node| node.is_small())
            .collect::<Vec<_>>();
        let small_nodes_set = small_nodes.iter().collect::<HashSet<_>>();

        small_nodes.len() > small_nodes_set.len()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.0 {
            f.write_str(&s.0)?;
            f.write_char('-')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_lines("inputs/day12-test.txt")?;
        let matrix = super::Graph::new(&input)?;
        assert_eq!(matrix.part_one(), 19);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_lines("inputs/day12.txt")?;
        let matrix = super::Graph::new(&input)?;
        assert_eq!(matrix.part_one(), 3463);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_lines("inputs/day12-test.txt")?;
        let matrix = super::Graph::new(&input)?;
        assert_eq!(matrix.part_two(), 103);
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_lines("inputs/day12.txt")?;
        let matrix = super::Graph::new(&input)?;
        assert_eq!(matrix.part_two(), 91533);
        Ok(())
    }
}
