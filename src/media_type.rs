use parser::parse_priorities_for;
use super::common::sorter;

pub fn matching_media_types<S: Str>(maybe_accepts: Option<String>,
                                 provided: Vec<S>) -> Vec<S> {
    // Parse the header.
    // RFC 2616 says no header is equivalent to *
    let accepts = maybe_accepts.unwrap_or("*/*".to_string());

    // Get the priorities for all provided options
    let mut media_types = parse_priorities_for(accepts, provided);

    // Sort by priority
    media_types.sort_by(sorter);

    // Get all the media_types and return
    media_types.move_iter().map(|(c, _)| c).collect()
}

