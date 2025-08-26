use std::collections::HashMap;

mod raw;

struct FeaturedWord {
    word: &'static str,
    prefix: Option<&'static str>,
    suffix: Option<&'static str>,
    sufsuffix: Option<&'static str>,
}

lazy
