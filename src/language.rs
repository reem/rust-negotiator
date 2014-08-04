use parser::parse_priorities_for;
use super::common::sorter;

pub fn matching_languages<S: Str>(maybe_accepts: Option<String>,
                                 provided: Vec<S>) -> Vec<S> {
    // Parse the header.
    // RFC 2616 says no header is equivalent to *
    let accepts = maybe_accepts.unwrap_or("*/*".to_string());

    // Get the priorities for all provided options
    let mut languages = parse_priorities_for(accepts, provided);

    // Sort by priority
    languages.sort_by(sorter);

    // Get all the languages and return
    languages.move_iter().map(|(c, _)| c).collect()
}

