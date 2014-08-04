pub fn sorter<S>(&(_, priority_a): &(S, f64),
                 &(_, priority_b): &(S, f64)) -> Ordering {
    // Revsort
    priority_b.partial_cmp(&priority_a).unwrap()
}
