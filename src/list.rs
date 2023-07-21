use std::fmt::Display;

use itertools::join;

/// A singly-linked list with owned nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum List<T> {
    Node(T, Box<List<T>>),
    Nil,
}

impl<T> Display for List<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Node(_, _) => {
                write!(f, "[")?;
                write!(f, "{}", join(self.clone(), ", "))?;
                write!(f, "]")
            }
            List::Nil => write!(f, "[]"),
        }
    }
}

impl<T: Clone> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            List::Node(data, next) => {
                let data = data.clone();
                *self = *next.clone();
                Some(data)
            }
            List::Nil => None,
        }
    }
}

impl<T: Clone> ExactSizeIterator for List<T> {
    fn len(&self) -> usize {
        self.clone().fold(0, |acc, _| acc + 1)
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(Self::Nil, |list, data| List::Node(data, Box::new(list)))
    }
}
