#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(unnecessary_qualification, non_camel_case_types,
        unnecessary_typecast)]
#![feature(phase, globs)]

//! Framework-agnostic HTTP Content Negotiation.

pub trait Negotiable {
    fn get_charset(&self) -> Option<String> { None }
    fn get_encoding(&self) -> Option<String> { None }
    fn get_language(&self) -> Option<String> { None }
    fn get_media_type(&self) -> Option<String> { None }

    fn charset<S: Str>(&self, accepts: Vec<S>) -> Option<String> {
        self.charsets(accepts).and_then(|v| v.move_iter().next())
    }

    fn charsets<S: Str>(&self, accepts: Vec<S>) -> Option<Vec<String>> {
        None
    }

    fn encoding<S: Str>(&self, accepts: Vec<S>) -> Option<String> {
        self.encodings(accepts).and_then(|v| v.move_iter().next())
    }

    fn encodings<S: Str>(&self, accepts: Vec<S>) -> Option<Vec<String>> {
        None
    }

    fn language<S: Str>(&self, accepts: Vec<S>) -> Option<String> {
        self.languages(accepts).and_then(|v| v.move_iter().next())
    }

    fn languages<S: Str>(&self, accepts: Vec<S>) -> Option<Vec<String>> {
        None
    }

    fn media_type<S: Str>(&self, accepts: Vec<S>) -> Option<String> {
        self.media_types(accepts).and_then(|v| v.move_iter().next())
    }

    fn media_types<S: Str>(&self, accepts: Vec<S>) -> Option<Vec<String>> {
        None
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

