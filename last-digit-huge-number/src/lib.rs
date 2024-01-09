// Remove all unnecessary 0's and 1's from the sequence
fn compact(arr: &[u64]) -> Vec<u64> {
    let mut elems = vec![];
    for e in arr.iter().rev() {
        if elems.is_empty() {
            elems.push(*e)
        } else if elems.last().unwrap() == &0 {
            elems.pop();
            elems.push(1);
        } else if e == &0 || e == &1 {
            elems.clear();
            elems.push(*e);
        } else {
            elems.push(*e);
        }
    }

    elems.reverse();

    if elems.len() > 1 {
        if let Some(index) = elems.iter().position(|&x| x == 0 || x == 1) {
            elems.resize(index, 0);
        }
    }

    elems
}

// Calculate a0^(a1^(...)) mod 4
// a_i > 1 for all i > 0
fn powers_mod4(arr: &[u64]) -> u64 {
    let a0 = arr[0] % 4;
    if arr.len() == 1 {
        return a0;
    }

    match a0 {
        0 => 0,
        1 => 1,
        2 => 0, // a[1] > 1, so 2^(a[1] ^ ...) mod 4 = 0
        _ => {
            // 3
            if arr[1] % 2 == 0 {
                1
            } else {
                3
            }
        }
    }
}

pub fn last_digit(arr: &[u64]) -> u64 {
    let arr = compact(arr);
    if arr.is_empty() {
        return 1;
    }

    if arr.len() == 1 || [0, 1, 5, 6].contains(&(arr[0] % 10)) {
        return arr[0] % 10;
    }

    // x ^ (4k + m) = x ^ m mod 10
    u64::pow(arr[0] % 10, powers_mod4(&arr[1..]) as u32 + 4) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(v: &[u64], expected: u64) {
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (a, b) in [
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![2, 2, 101, 2], 6),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6),
            (
                vec![
                    186719, 665559, 388931, 189780, 223543, 526003, 683264, 726766, 638799, 706579,
                ],
                9,
            ),
        ] {
            dotest(&a, b);
        }
    }
}
