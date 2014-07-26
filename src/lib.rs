#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(unnecessary_qualification, non_camel_case_types,
         unused_variable, unnecessary_typecast)]
#![feature(phase, globs)]

//! Framework-agnostic HTTP Content Negotiation.

pub trait Negotiable {
    fn charset(&self) -> Option<String> { None }
    fn encoding(&self) -> Option<String> { None }
    fn language(&self) -> Option<String> { None }
    fn media_type(&self) -> Option<String> { None }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Negotiable for String {
        fn charset(&self) -> Option<String> { Some(self.clone()) }
        fn encoding(&self) -> Option<String> { Some(self.clone()) }
        fn language(&self) -> Option<String> { Some(self.clone()) }
        fn media_type(&self) -> Option<String> { Some(self.clone()) }
    }

    impl<'a> Negotiable for &'a str {
        fn charset(&self) -> Option<String> { Some(self.to_string()) }
        fn encoding(&self) -> Option<String> { Some(self.to_string()) }
        fn language(&self) -> Option<String> { Some(self.to_string()) }
        fn media_type(&self) -> Option<String> { Some(self.to_string()) }
    }
}

