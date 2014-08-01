#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]
#![feature(phase, globs)]

//! Framework-agnostic HTTP Content Negotiation.
//!
//! Heavily influenced by https://github.com/federomero/negotiator

extern crate parser = "http-parse-priority-header";

use self::charset::matching_charsets;
use self::encoding::matching_encodings;
//use self::language::matching_languages;
//use self::media_type::matching_media_types;

mod charset;
mod encoding;
mod language;
mod media_type;

pub trait Negotiable {
    fn get_charset(&self) -> Option<String> { None }
    fn get_encoding(&self) -> Option<String> { None }
    fn get_language(&self) -> Option<String> { None }
    fn get_media_type(&self) -> Option<String> { None }

    fn charset<S: Str>(&self, provided: Vec<S>) -> Option<S> {
        self.charsets(provided).move_iter().next()
    }

    fn charsets<S: Str>(&self, provided: Vec<S>) -> Vec<S> {
        matching_charsets(self.get_charset(), provided)
    }

    fn encoding<S: Str>(&self, provided: Vec<S>) -> Option<S> {
        self.encodings(provided).move_iter().next()
    }

    fn encodings<S: Str>(&self, provided: Vec<S>) -> Vec<S> {
        matching_encodings(self.get_encoding(), provided)
    }

    fn language<S: Str>(&self, provided: Vec<S>) -> Option<S> {
        self.languages(provided).move_iter().next()
    }

    fn languages<S: Str>(&self, provided: Vec<S>) -> Vec<S> {
        vec![]
    }

    fn media_type<S: Str>(&self, provided: Vec<S>) -> Option<S> {
        self.media_types(provided).move_iter().next()
    }

    fn media_types<S: Str>(&self, provided: Vec<S>) -> Vec<S> {
        vec![]
    }

}

#[cfg(test)]
mod test {
    use super::*;

    impl Negotiable for String {
        fn get_charset(&self) -> Option<String> { Some(self.clone()) }
        fn get_encoding(&self) -> Option<String> { Some(self.clone()) }
        fn get_language(&self) -> Option<String> { Some(self.clone()) }
        fn get_media_type(&self) -> Option<String> { Some(self.clone()) }
    }

    impl<'a> Negotiable for &'a str {
        fn get_charset(&self) -> Option<String> { Some(self.to_string()) }
        fn get_encoding(&self) -> Option<String> { Some(self.to_string()) }
        fn get_language(&self) -> Option<String> { Some(self.to_string()) }
        fn get_media_type(&self) -> Option<String> { Some(self.to_string()) }
    }
}

