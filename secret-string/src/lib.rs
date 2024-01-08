use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Graph using adjacency list representation
struct Graph<T> {
    nodes: HashMap<T, Vec<T>>,
}

impl<T: Eq + Hash + Copy> Graph<T> {
    fn new() -> Self {
        Self {
            nodes: HashMap::<T, Vec<T>>::new(),
        }
    }

    /// Insert a directed edge to the graph
    fn insert(&mut self, start_node: T, end_node: T) {
        let entry = self.nodes.entry(start_node).or_default();
        if !entry.contains(&end_node) {
            entry.push(end_node)
        }
        self.nodes.entry(end_node).or_default();
    }

    /// Depth-first-search from a given node
    fn dfs(&self, n: T, visited: &mut HashSet<T>, sorted: &mut Vec<T>) {
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
    fn topological_sort(&self) -> Vec<T> {
        let mut visited = HashSet::<T>::new();
        let mut sorted = Vec::<T>::new();

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
