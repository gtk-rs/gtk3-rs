// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// This is similar to the GVariantIter provided by glib, but that would
// introduce a heap allocation and doesn't provide a way to determine how
// many items are left in the iterator.

use std::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};

use variant::Variant;

/// Iterator over items in a variant.
#[derive(Debug)]
pub struct VariantIter {
    variant: Variant,
    head: usize,
    tail: usize,
}

impl VariantIter {
    pub(crate) fn new(variant: Variant) -> Self {
        let tail = variant.n_children();
        Self {
            variant,
            head: 0,
            tail,
        }
    }
}

impl Iterator for VariantIter {
    type Item = Variant;

    fn next(&mut self) -> Option<Variant> {
        if self.head == self.tail {
            None
        } else {
            let value = self.variant.get_child_value(self.head);
            self.head += 1;
            Some(value)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.tail - self.head;
        (size, Some(size))
    }
}

impl DoubleEndedIterator for VariantIter {
    fn next_back(&mut self) -> Option<Variant> {
        if self.head == self.tail {
            None
        } else {
            self.tail -= 1;
            Some(self.variant.get_child_value(self.tail))
        }
    }
}

impl ExactSizeIterator for VariantIter {}

#[cfg(test)]
mod tests {
    use prelude::*;
    use std::collections::HashMap;
    use variant::DictEntry;
    use variant::Variant;

    #[test]
    fn test_variant_iter_variant() {
        let v = Variant::variant(&"foo".to_string().to_variant());
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string()]);
    }

    #[test]
    fn test_variant_iter_array() {
        let v = Variant::array::<String>(&vec![
            "foo".to_string().to_variant(),
            "bar".to_string().to_variant(),
        ]);
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_variant_iter_tuple() {
        let v = Variant::tuple(&vec![
            "foo".to_string().to_variant(),
            "bar".to_string().to_variant(),
        ]);
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_variant_iter_dictentry() {
        let v = DictEntry::new("foo", 1337).to_variant();
        println!("{:?}", v.iter().collect::<Vec<_>>());
        assert_eq!(v.iter().count(), 2);
    }

    #[test]
    fn test_variant_iter_map() {
        let mut map = HashMap::new();
        map.insert("foo", 1);
        map.insert("bar", 1);
        let v = map.to_variant();
        assert_eq!(v.iter().count(), 2);
    }
}
