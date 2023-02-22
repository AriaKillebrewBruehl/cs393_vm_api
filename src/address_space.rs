use std::collections::LinkedList;
use std::iter::Map;
use std::sync::Arc;

use crate::data_source::DataSource;

type VirtualAddress = usize;

struct MapEntry {
    source: Arc<dyn DataSource>,
    offset: usize,
    span: usize,
    addr: usize,
}

/// An address space.
pub struct AddressSpace {
    name: String,
    mappings: LinkedList<MapEntry>, // see below for comments
}

// comments about storing mappings
// Most OS code uses doubly-linked lists to store sparse data structures like
// an address space's mappings.
// Using Rust's built-in LinkedLists is fine. See https://doc.rust-lang.org/std/collections/struct.LinkedList.html
// But if you really want to get the zen of Rust, this is a really good read, written by the original author
// of that very data structure: https://rust-unofficial.github.io/too-many-lists/

// So, feel free to come up with a different structure, either a classic Rust collection,
// from a crate (but remember it needs to be #no_std compatible), or even write your own.

impl AddressSpace {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mappings: LinkedList::new(),
        }
    }

    /// Add a mapping from a `DataSource` into this `AddressSpace`.
    ///
    /// # Errors
    /// If the desired mapping is invalid.
    pub fn add_mapping<D: DataSource>(
        &self,
        source: &D,
        offset: usize,
        span: usize,
    ) -> Result<VirtualAddress, &str> {
        // todo!()
        let mut start_free = 0;
        let mut end_free = 2 ^ 39 - 1;
        let mut iter = self.mappings.iter();

        if iter.is_empty() {
            let src: Arc<dyn DataSource> = { source };
            let entry = MapEntry {
                source: src,
                offset: offset,
                span: span,
                addr: start_free,
            };
            self.mappings.push_back(entry);
            return Ok(entry.addr);
        }

        loop {
            if let Some(first_entry) = iter.next() {
                start_free = first_entry.addr + first_entry.span;
                if let Some(second_entry) = iter.next() {
                    end_free = second_entry.addr - 1;
                } else {
                    end_free = 2 ^ 39 - 1;
                }
            } else {
                return Err("cannot fit data source in address space");
            }
            if end_free - start_free >= span {
                let src: Arc<dyn DataSource> = { source };
                let entry = MapEntry {
                    source: src,
                    offset: offset,
                    span: span,
                    addr: start_free,
                };
                self.mappings.push_back(entry);
                return Ok(entry.addr);
            }
        }
    }

    /// Add a mapping from `DataSource` into this `AddressSpace` starting at a specific address.
    ///
    /// # Errors
    /// If there is insufficient room subsequent to `start`.
    pub fn add_mapping_at<D: DataSource>(
        &self,
        source: &D,
        offset: usize,
        span: usize,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        let mut start_free;
        let mut end_free;
        let mut iter = self.mappings.iter();

        if iter.is_empty() {
            let src: Arc<dyn DataSource> = { source };
            let entry = MapEntry {
                source: src,
                offset: offset,
                span: span,
                addr: start,
            };
            self.mappings.push_back(entry);
            return Ok(());
        }
        loop {
            if let Some(first_entry) = iter.next() {
                start_free = first_entry.addr + first_entry.span;
                if let Some(second_entry) = iter.next() {
                    end_free = second_entry.addr - 1;
                } else {
                    end_free = 2 ^ 39 - 1;
                }
            } else {
                return Err("cannot fit data source in address space");
            }
            if start_free <= start && start <= end_free {
                if start + span <= end_free {
                    let src: Arc<dyn DataSource> = { source };
                    let entry = MapEntry {
                        source: src,
                        offset: offset,
                        span: span,
                        addr: start_free,
                    };
                    self.mappings.push_back(entry);
                    return Ok(());
                } else {
                    return Err("cannot fit data source into address space");
                }
            }
        }
    }

    /// Remove the mapping to `DataSource` that starts at the given address.
    ///
    /// # Errors
    /// If the mapping could not be removed.
    pub fn remove_mapping<D: DataSource>(
        &self,
        source: &D,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        todo!()
    }
}
