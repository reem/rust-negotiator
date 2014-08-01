pub fn sorter<S: Str>(&(ref set_a, priority_a): &(S, f64),
                      &(ref set_b, priority_b): &(S, f64)) -> Ordering {
    // If these are the same charset, sort by priority
    if set_a.as_slice() == set_b.as_slice() {
        priority_a.partial_cmp(&priority_b).unwrap()
    } else {
        set_a.as_slice().cmp(&set_b.as_slice())
    }
}
