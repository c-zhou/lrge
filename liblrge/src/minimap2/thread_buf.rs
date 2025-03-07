//! The code in this module is adapted from the [`minimap2` crate](https://github.com/jguhlin/minimap2-rs).
use std::cell::RefCell;

use minimap2_sys::{mm_tbuf_destroy, mm_tbuf_init, mm_tbuf_t};

// Thread local buffer (memory management) for minimap2
thread_local! {
    pub(crate) static BUF: RefCell<ThreadLocalBuffer> = RefCell::new(ThreadLocalBuffer::new());
}

/// ThreadLocalBuffer for minimap2 memory management
#[derive(Debug)]
pub(crate) struct ThreadLocalBuffer {
    buf: *mut mm_tbuf_t,
    uses: usize,
}

impl ThreadLocalBuffer {
    pub fn new() -> Self {
        let buf = unsafe { mm_tbuf_init() };
        Self { buf, uses: 0 }
    }
    /// Return the buffer, checking how many times it has been borrowed.
    /// Free the memory of the old buffer and reinitialise a new one Ii num_uses exceeds max_uses.
    pub fn get_buf(&mut self) -> *mut mm_tbuf_t {
        self.uses += 1;
        self.buf
    }
}

/// Handle destruction of thread local buffer properly.
impl Drop for ThreadLocalBuffer {
    fn drop(&mut self) {
        unsafe { mm_tbuf_destroy(self.buf) };
    }
}

impl Default for ThreadLocalBuffer {
    fn default() -> Self {
        Self::new()
    }
}
