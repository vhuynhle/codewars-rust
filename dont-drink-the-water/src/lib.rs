use std::iter::{self, from_fn, Iterator};

fn chunk<I>(iter: impl IntoIterator<Item = I>, chunk_size: usize) -> impl Iterator<Item = Vec<I>> {
    let mut iter = iter.into_iter();
    from_fn(move || {
        let v: Vec<I> = iter.by_ref().take(chunk_size).collect();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    })
}

pub fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    if glass.is_empty() {
        return vec![];
    }

    let mut blob_counts = [0; 4];
    for orig_layer in glass {
        for blob in orig_layer {
            match *blob {
                'O' => blob_counts[0] += 1,
                'A' => blob_counts[1] += 1,
                'W' => blob_counts[2] += 1,
                _ => blob_counts[3] += 1,
            }
        }
    }

    let layer_width = glass[0].len();
    let iter = blob_counts
        .iter()
        .zip(&['O', 'A', 'W', 'H'])
        .flat_map(|(count, blob)| iter::repeat(*blob).take(*count));

    chunk(iter, layer_width).collect()
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
