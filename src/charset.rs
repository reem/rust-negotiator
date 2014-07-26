use std::collections::HashMap;
use regex::Regex;

static error: f64 = -1.0;

pub fn matching_charsets<S: Str>(maybe_accepts: Option<String>,
                                 provided: Vec<S>) -> Vec<S> {
    // Parse the header.
    // RFC 2616 says no header is equivalent to *
    let accepts = parse_charsets(
        maybe_accepts.as_ref().map(|a| a.as_slice()).unwrap_or("*")
    );

    // Get the priorities for all provided options
    let mut charsets = provided.move_iter().map(|possibility| {
        let priority = accepts.find_equiv(&possibility.as_slice()).unwrap_or(&error);
        (possibility, priority)

    // Remove all the possibilities that are not accepted
    }).filter(|&(_, &priority)| priority > 0.0)

    // Collect for sorting
    .collect::<Vec<(S, &f64)>>();

    // Sort alphabetically, then by priority
    charsets.sort_by(|&(ref set_a, &priority_a), &(ref set_b, &priority_b)| {
        // If these are the same charset, sort by priority
        if set_a.as_slice() == set_b.as_slice() {
            priority_a.partial_cmp(&priority_b).unwrap()
        } else {
            set_a.as_slice().cmp(&set_b.as_slice())
        }
    });

    // Get all the charsets and return
    charsets.move_iter().map(|(c, _)| c).collect()
}

fn parse_charsets<'a>(charsets: &'a str) -> HashMap<&'a str, f64> {
    charsets
        .trim_chars(whitespace)
        .split(',')
        .filter_map(parse_charset).collect()
}

static is_charset: Regex = regex!(r"/^\s*(\S+?)\s*(?:;(.*))?$/");
static whitespace: &'static [char] = &[' ', '\t', '\n'];

fn parse_charset(charset: &str) -> Option<(&str, f64)> {
    is_charset.captures(charset).map(|captures| {
        let charset = captures.name("set");

        let q = captures.name("params").split(';').find(|&param| {
            // Is this param q?
            param.trim_chars(whitespace)
                .split('=')
                .collect::<Vec<&str>>()[0] == "q"
        // No q found, so default to 1.0
        }).map_or(1.0, |param| {
            // Found q, parse its value
            from_str::<f64>(param.trim_chars(whitespace)
                .split('=')
                .collect::<Vec<&str>>()[1])
        // Failed to parse q value, default to -1
        }.unwrap_or(-1.0));

        (charset, q)
    })
}

