use std::collections::HashMap;

pub fn partitions(n: u32) -> usize {
    let mut cache = HashMap::<(u32, u32), usize>::new();
    for i in 0..=n {
        cache.insert((i, 1), 1);
        cache.insert((0, i), 1);
    }

    return cached_partitions(n, n, &mut cache);
}

fn cached_partitions(n: u32, component_cap: u32, cache: &mut HashMap<(u32, u32), usize>) -> usize {
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

    return res;
}

#[cfg(test)]
mod tests {
    use super::partitions;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }

    #[test]
    fn basic_test_200() {
        assert_eq!(partitions(200),  3972999029388);
    }
}
