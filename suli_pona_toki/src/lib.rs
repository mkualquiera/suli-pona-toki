pub mod content;
pub mod preposition;
pub mod sentence;
pub mod tokens;

pub trait Natural {
    fn as_natural(&self) -> String;
}
