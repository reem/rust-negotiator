use parser::parse_priorities_for;
use super::common::sorter;

pub fn matching_encodings<S: Str>(maybe_accepts: Option<String>,
                                 provided: Vec<S>) -> Vec<S> {
    // Parse the header.
    // RFC 2616 says no header is equivalent to *
    let accepts = match maybe_accepts {
        Some(d) => d,
        None => return vec![]
    };

    // Get the priorities for all provided options
    let mut encodings = parse_priorities_for(accepts, provided);

    // Sort by priority
    encodings.sort_by(sorter);

    // FIXME: Adding identity is waiting on a from_slice trait that allows
    // us to add to encodings.

    // Get all the encodings and return
    encodings.move_iter().map(|(c, _)| c).collect()
}

