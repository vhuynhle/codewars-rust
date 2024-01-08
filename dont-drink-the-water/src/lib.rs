use std::iter::Iterator;

pub fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    if glass.is_empty() {
        return vec![];
    }

    let layer_width = glass[0].len();
    let mut blobs = glass
        .iter()
        .flat_map(|v| v.iter().cloned())
        .collect::<Vec<char>>();

    blobs.sort_by_key(|&k| match k {
        'H' => 136, // Honey
        'W' => 100, // Water
        'A' => 87,  // Alcohol
        'O' => 80,  // Oil
        _ => 0,     // Unknown substance
    });

    blobs.chunks(layer_width).map(|v| v.to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::separate_liquids;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[Vec<char>], expected: &[Vec<char>]) {
        assert_eq!(
            separate_liquids(a),
            expected,
            "{ERR_MSG} with glass = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                vec!['H', 'H', 'W', 'O'],
                vec!['W', 'W', 'O', 'W'],
                vec!['H', 'H', 'O', 'O'],
            ],
            &[
                vec!['O', 'O', 'O', 'O'],
                vec!['W', 'W', 'W', 'W'],
                vec!['H', 'H', 'H', 'H'],
            ],
        );
        dotest(
            &[
                vec!['A', 'A', 'O', 'H'],
                vec!['A', 'H', 'W', 'O'],
                vec!['W', 'W', 'A', 'W'],
                vec!['H', 'H', 'O', 'O'],
            ],
            &[
                vec!['O', 'O', 'O', 'O'],
                vec!['A', 'A', 'A', 'A'],
                vec!['W', 'W', 'W', 'W'],
                vec!['H', 'H', 'H', 'H'],
            ],
        );
        dotest(&[vec!['A', 'H', 'W', 'O']], &[vec!['O', 'A', 'W', 'H']]);
        dotest(
            &[vec!['A'], vec!['H'], vec!['W'], vec!['O']],
            &[vec!['O'], vec!['A'], vec!['W'], vec!['H']],
        );
        dotest(&[], &[]);
    }
}
