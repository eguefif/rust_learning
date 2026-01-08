use memmap2::{MmapMut, MmapOptions};
use std::cell::{Cell, RefCell};

pub struct Arena {
    buffer: RefCell<MmapMut>,
    offset: Cell<usize>,
    buffer_len: usize,
}

impl Arena {
    pub fn new(size: usize) -> std::io::Result<Self> {
        let buffer = MmapOptions::new().len(size).map_anon()?;
        Ok(Self {
            buffer: RefCell::new(buffer),
            offset: Cell::new(0),
            buffer_len: size,
        })
    }

    // TODO: check what happens when we try to allocate too much
    pub fn alloc<'arena, T: Copy>(&'arena self, value: T) -> &'arena mut T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let new_offset = (self.offset.get() + align - 1) & !(align - 1);

        let x_ptr = self.buffer.borrow_mut().as_mut_ptr().cast::<T>();
        if new_offset + size > self.buffer_len {
            todo!("Handle error when buffer is full");
        }
        let ptr = unsafe {
            let ptr = x_ptr.offset(new_offset as isize);
            *ptr = value;
            ptr
        };
        self.offset.set(new_offset + size);

        unsafe { &mut *ptr }
    }

    pub fn reset(&mut self) {
        self.offset.set(0);
    }
}
