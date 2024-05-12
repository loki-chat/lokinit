/// Provides the contents for a [`Surface`].
///
/// [`Surface`]: super::surface::Surface
pub struct Buffer {}
impl Buffer {
    /// Destroys the buffer. The buffer is immediately invalidated after this call;
    /// it cannot be mutated or destroyed.
    pub fn destroy(self) {
        todo!()
    }
}

pub trait BufferListener {
    /// Indicates that the buffer is no longer being used by the compositor, so the
    /// client is free to mutate or destroy it.
    fn release(&self);
}
