use std::collections::{HashMap, HashSet};

/// Graph using adjacency list representation
struct Graph {
    nodes: HashMap<char, Vec<char>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::<char, Vec<char>>::new(),
        }
    }

    /// Insert a directed edge to the graph
    fn insert(&mut self, c1: char, c2: char) {
        let entry = self.nodes.entry(c1).or_default();
        if !entry.contains(&c2) {
            entry.push(c2)
        }
        self.nodes.entry(c2).or_default();
    }

    /// Depth-first-search from a given node
    fn dfs(&self, n: char, visited: &mut HashSet<char>, sorted: &mut Vec<char>) {
        if visited.contains(&n) {
            return;
        }

        for neighbor in self.nodes.get(&n).unwrap() {
            if !visited.contains(neighbor) {
                self.dfs(*neighbor, visited, sorted);
            }
        }
        visited.insert(n);
        sorted.push(n);
    }

    /// Perform topological sort
    fn topological_sort(&self) -> Vec<char> {
        let mut visited = HashSet::<char>::new();
        let mut sorted = Vec::<char>::new();

        for node in self.nodes.keys() {
            self.dfs(*node, &mut visited, &mut sorted)
        }

        sorted.reverse();
        sorted
    }
}

pub fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut graph = Graph::new();
    for triplet in triplets {
        graph.insert(triplet[0], triplet[1]);
        graph.insert(triplet[1], triplet[2]);
    }

    let sorted = graph.topological_sort();
    sorted.iter().collect()
}

#[cfg(test)]
mod test {
    use super::recover_secret;

    #[test]
    fn example_test() {
        assert_eq!(
            recover_secret(vec![
                ['t', 'u', 'p'],
                ['w', 'h', 'i'],
                ['t', 's', 'u'],
                ['a', 't', 's'],
                ['h', 'a', 'p'],
                ['t', 'i', 's'],
                ['w', 'h', 's']
            ]),
            "whatisup"
        );
    }
}
