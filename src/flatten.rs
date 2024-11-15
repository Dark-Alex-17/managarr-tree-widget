use crate::tree_item::TreeItem;
use ratatui::text::ToText;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

/// A flattened item of all visible [`TreeItem`]s.
///
/// Generated via [`TreeState::flatten`](crate::TreeState::flatten).
#[must_use]
pub struct Flattened<'a, T>
where
    T: ToText + Clone + Default + Display + Hash + PartialEq + Eq,
{
    pub identifier: Vec<u64>,
    pub item: &'a TreeItem<T>,
}

impl<'a, T> Flattened<'a, T>
where
    T: ToText + Clone + Default + Display + Hash + PartialEq + Eq,
{
    /// Zero based depth. Depth 0 means top level with 0 indentation.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.identifier.len() - 1
    }
}

/// Get a flat list of all visible [`TreeItem`]s.
///
/// `current` starts empty: `&[]`
#[must_use]
pub fn flatten<'a, T>(
    open_identifiers: &HashSet<Vec<u64>>,
    items: &'a [TreeItem<T>],
    current: &[u64],
) -> Vec<Flattened<'a, T>>
where
    T: ToText + Clone + Default + Display + Hash + PartialEq + Eq,
{
    let mut result = Vec::new();
    for item in items {
        let mut child_identifier = current.to_vec();
        child_identifier.push(item.identifier);

        let child_result = open_identifiers
            .contains(&child_identifier)
            .then(|| flatten(open_identifiers, &item.children, &child_identifier));

        result.push(Flattened {
            identifier: child_identifier,
            item,
        });

        if let Some(mut child_result) = child_result {
            result.append(&mut child_result);
        }
    }
    result
}

#[test]
fn depth_works() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut open = HashSet::new();
    let hash = |s: &str| {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };
    open.insert(vec![hash("Bravo")]);
    open.insert(vec![hash("Bravo"), hash("Delta")]);
    let depths = flatten(&open, &TreeItem::example(), &[])
        .into_iter()
        .map(|flattened| flattened.depth())
        .collect::<Vec<_>>();
    assert_eq!(depths, [0, 0, 1, 1, 2, 2, 1, 0]);
}

#[cfg(test)]
fn flatten_works(open: &HashSet<Vec<u64>>, expected: &[u64]) {
    let items = TreeItem::example();
    let result = flatten(open, &items, &[]);
    let actual = result
        .into_iter()
        .map(|flattened| flattened.identifier.into_iter().last().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn flatten_nothing_open_is_top_level() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let open = HashSet::new();
    let hash = |s: &str| {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };
    flatten_works(&open, &[hash("Alfa"), hash("Bravo"), hash("Hotel")]);
}

#[test]
fn flatten_wrong_open_is_only_top_level() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut open = HashSet::new();
    let hash = |s: &str| {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };
    open.insert(vec![hash("Alfa")]);
    open.insert(vec![hash("Bravo"), hash("Delta")]);
    flatten_works(&open, &[hash("Alfa"), hash("Bravo"), hash("Hotel")]);
}

#[test]
fn flatten_one_is_open() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut open = HashSet::new();
    let hash = |s: &str| {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };
    open.insert(vec![hash("Bravo")]);
    flatten_works(
        &open,
        &[
            hash("Alfa"),
            hash("Bravo"),
            hash("Charlie"),
            hash("Delta"),
            hash("Golf"),
            hash("Hotel"),
        ],
    );
}

#[test]
fn flatten_all_open() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut open = HashSet::new();
    let hash = |s: &str| {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };
    open.insert(vec![hash("Bravo")]);
    open.insert(vec![hash("Bravo"), hash("Delta")]);
    flatten_works(
        &open,
        &[
            hash("Alfa"),
            hash("Bravo"),
            hash("Charlie"),
            hash("Delta"),
            hash("Echo"),
            hash("Foxtrot"),
            hash("Golf"),
            hash("Hotel"),
        ],
    );
}
