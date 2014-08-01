use parser::parse_priorities_for;
use super::common::sorter;

pub fn matching_charsets<S: Str>(maybe_accepts: Option<String>,
                                 provided: Vec<S>) -> Vec<S> {
    // Parse the header.
    // RFC 2616 says no header is equivalent to *
    let accepts = maybe_accepts.unwrap_or("*".to_string());

    // Get the priorities for all provided options
    let mut charsets = parse_priorities_for(accepts, provided);

    // Sort alphabetically, then by priority
    charsets.sort_by(sorter);

    // Get all the charsets and return
    charsets.move_iter().map(|(c, _)| c).collect()
}

