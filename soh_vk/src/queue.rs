use ash::vk;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Queue {
    queue: vk::Queue,
}

// Constructor, destructor
impl Queue {
    pub fn new(queue: vk::Queue) -> Self {
        return Queue { queue };
    }

    pub fn null() -> Self {
        return Queue {
            queue: vk::Queue::null(),
        };
    }
}

// Deref
impl std::ops::Deref for Queue {
    type Target = vk::Queue;

    fn deref(&self) -> &Self::Target {
        return &self.queue;
    }
}
