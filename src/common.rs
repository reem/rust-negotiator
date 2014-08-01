pub fn sorter<S>(&(_, priority_a): &(S, f64),
                 &(_, priority_b): &(S, f64)) -> Ordering {
    priority_a.partial_cmp(&priority_b).unwrap()
}
