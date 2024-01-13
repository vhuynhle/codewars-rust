use std::collections::HashMap;

pub fn exp_sum(n: u64) -> u64 {
    let mut cache = HashMap::<(u64, u64), u64>::new();
    for i in 0..=n {
        cache.insert((i, 1), 1);
        cache.insert((0, i), 1);
    }

    cached_partitions(n, n, &mut cache)
}

fn cached_partitions(n: u64, component_cap: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if cache.contains_key(&(n, component_cap)) {
        return *cache.get(&(n, component_cap)).unwrap();
    }

    if component_cap > n {
        return cached_partitions(n, n, cache);
    }

    let partitions_same_cap = cached_partitions(n - component_cap, component_cap, cache);
    let partitions_with_smaller_cap = cached_partitions(n, component_cap - 1, cache);
    let res = partitions_same_cap + partitions_with_smaller_cap;
    cache.insert((n, component_cap), res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sample_tests() {
        assert_eq!(exp_sum(1), 1);
        assert_eq!(exp_sum(2), 2);
        assert_eq!(exp_sum(3), 3);
        assert_eq!(exp_sum(4), 5);
        assert_eq!(exp_sum(5), 7);
        assert_eq!(exp_sum(10), 42);
    }
}
