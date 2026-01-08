use memmap2::{MmapMut, MmapOptions};
use std::cell::{Cell, RefCell};

const DEFAULT_SIZE: usize = 4096;

pub struct Arena {
    buffer: RefCell<Vec<MmapMut>>,
    offset: Cell<usize>,
    buffer_offset: Cell<usize>,
}

impl Arena {
    pub fn new() -> std::io::Result<Self> {
        let buffer = MmapOptions::new().len(DEFAULT_SIZE).map_anon()?;
        Ok(Self {
            buffer: RefCell::new(vec![buffer]),
            offset: Cell::new(0),
            buffer_offset: Cell::new(0),
        })
    }

    // TODO: Test allocation
    //      * Normal alloc
    //      * When passing buffer
    //      * After reset
    //
    // TODO: think of what if the allocation is bigger than the default size
    pub fn alloc<'arena, T>(&'arena self, value: T) -> std::io::Result<&'arena mut T> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let new_offset = (self.offset.get() + align - 1) & !(align - 1);

        let mut buffers = self.buffer.borrow_mut();
        if new_offset + size > DEFAULT_SIZE {
            self.realloc(&mut buffers)?;
        }

        let x_ptr = buffers
            .get_mut(self.buffer_offset.get())
            .expect("Should return a buffer")
            .as_mut_ptr()
            .cast::<T>();

        let ptr = unsafe {
            let ptr = x_ptr.offset(new_offset as isize);
            *ptr = value;
            ptr
        };
        self.offset.set(new_offset + size);

        Ok(unsafe { &mut *ptr })
    }

    fn realloc(&self, buffers: &mut Vec<MmapMut>) -> std::io::Result<()> {
        let new_buffer = MmapOptions::new().len(DEFAULT_SIZE).map_anon()?;
        buffers.push(new_buffer);
        let current_buffer_offset = self.buffer_offset.get();
        self.buffer_offset.set(current_buffer_offset + 1);
        self.offset.set(0);
        Ok(())
    }

    pub fn reset(&self) {
        self.offset.set(0);
        self.buffer_offset.set(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_allocate_one_buffer() {
        let arena = Arena::new().expect("Error creating Arena");
        let x = arena.alloc(10).expect("Erro allocating 10");
        let y = arena.alloc("Hello, World").expect("Error allocating str");

        assert_eq!(&10, x);
        assert_eq!(&"Hello, World", y);
    }

    // TODO: Make that test pass
    #[test]
    fn it_should_allocate_a_second_buffer() {
        let arena = Arena::new().expect("Error creating Arena");

        for _ in 0..(4096 / 8 + 1) {
            let _ = arena.alloc(8u64);
        }
        assert_eq!(arena.buffer_offset.get(), 1);
    }
}
