pub struct GenArena<T> {
    elements: Vec<GenArenaElem<T>>,
    capacity: usize,
}

pub enum GenArenaElem<T> {
    Free,
    Occupied(T),
}

impl<T> GenArena<T> {
    pub fn new(capacity: usize) -> Self {
        let mut new_arena = GenArena {
            elements: Vec::new(),
            capacity,
        };
        new_arena.elements.reserve_exact(capacity);

        for i in 0..capacity {
            new_arena.elements[i] = GenArenaElem::Free;
        }
        new_arena
    }

    pub fn insert(elem: T) {}

    pub fn remove(index: u32) {}
}
