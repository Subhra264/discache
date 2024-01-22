/// Represents a generational arena with a fixed capacity.
pub struct GenArena<T> {
    elements: Vec<GenArenaElem<T>>,
    capacity: usize,
    free_head: Option<usize>,
    len: usize,
}

/// Defines an element for the [`GenArena`] arena.
pub enum GenArenaElem<T> {
    Free { next: Option<usize> },
    Occupied(T),
}

impl<T> GenArena<T> {
    /// Creates a new [`GenArena`] with default `capacity` of `10`.
    pub fn new() -> Self {
        GenArena::with_capacity(10)
    }

    /// Creates a new [`GenArena`] with the given `capacity`, default to `10`.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut new_arena = GenArena {
            elements: Vec::new(),
            capacity: if capacity == 0 { 10 } else { capacity },
            free_head: Some(0),
            len: 0,
        };
        new_arena.elements.reserve_exact(new_arena.capacity);

        for i in 0..new_arena.capacity {
            new_arena
                .elements
                .push(GenArenaElem::Free { next: Some(i + 1) });
        }
        new_arena.elements[new_arena.capacity - 1] = GenArenaElem::Free { next: None };
        new_arena
    }

    /// Returns the total number of occupied elements in the arena.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the arena is full.
    pub fn is_full(&self) -> bool {
        if self.free_head == None {
            true
        } else {
            false
        }
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        if index < self.capacity {
            match &self.elements[index] {
                GenArenaElem::Occupied(element) => Some(element),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.capacity {
            match &mut self.elements[index] {
                GenArenaElem::Occupied(element) => Some(element),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Pushes the given `elem` into the arena and returns its index.
    ///
    /// Returns Error if the arena is full.
    pub fn push(&mut self, elem: T) -> Result<usize, &'static str> {
        if !self.is_full() {
            let free_head = self.free_head.unwrap();
            match self.elements[free_head] {
                GenArenaElem::Free { next } => {
                    self.elements[free_head] = GenArenaElem::Occupied(elem);
                    self.free_head = next;
                    self.len += 1;
                }
                _ => unreachable!(),
            }
            Ok(free_head)
        } else {
            Err("Arena is full!")
        }
    }

    /// Removes the element at the given index.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.capacity {
            match &self.elements[index] {
                GenArenaElem::Occupied(_) => {
                    let free_entry = if let Some(free_head) = self.free_head {
                        GenArenaElem::Free {
                            next: Some(free_head),
                        }
                    } else {
                        GenArenaElem::Free { next: None }
                    };
                    let removed_bottom = std::mem::replace(&mut self.elements[index], free_entry);
                    self.free_head = Some(index);
                    self.len -= 1;
                    match removed_bottom {
                        GenArenaElem::Occupied(elem) => Some(elem),
                        _ => unreachable!(),
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns a iterator to iterate over the occupied elements.
    ///
    /// Note: Iteration may not be sequential.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        GenArenaIterator {
            arena: self,
            index: 0,
        }
    }
}

struct GenArenaIterator<'a, T> {
    arena: &'a GenArena<T>,
    index: usize,
}

impl<'a, T> Iterator for GenArenaIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= self.arena.len() - 1 {
            let mut elem_count = 0;
            for elem in self.arena.elements.iter() {
                match elem {
                    GenArenaElem::Occupied(element) => {
                        if elem_count == self.index {
                            self.index += 1;
                            return Some(element);
                        }
                        elem_count += 1;
                    }
                    _ => {}
                }
            }
        }
        return None;
    }
}
