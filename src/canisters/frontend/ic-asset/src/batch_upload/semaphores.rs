use futures_intrusive::sync::SharedSemaphore;

// Maximum MB of file data to load at once.  More memory may be used, due to encodings.
const MAX_SIMULTANEOUS_LOADED_MB: usize = 50;

// How many simultaneous chunks being created at once
const MAX_SIMULTANEOUS_CREATE_CHUNK: usize = 50;

// How many simultaneous Agent.call() to create_chunk(s)
const MAX_SIMULTANEOUS_CREATE_CHUNK_CALLS: usize = 25;

// How many simultaneous Agent.wait() on create_chunk(s) result
const MAX_SIMULTANEOUS_CREATE_CHUNK_WAITS: usize = 25;

#[derive(Debug)]
pub(crate) struct Semaphores {
    // The "file" semaphore limits how much file data to load at once.  A given loaded file's data
    // may be simultaneously encoded (gzip and so forth).
    pub file: SharedSemaphore,

    // The create_chunk semaphore limits the number of chunk creation calls
    // that can be in progress at one time.  Since each chunk creation can involve retries,
    // this focuses those retries on a smaller number of calls.
    // Without this semaphore, every chunk would make its first attempt, before
    // any chunk made its second attempt.
    pub create_chunk: SharedSemaphore,

    // The create_chunk_call semaphore limits the number of simultaneous
    // agent.call()s to create_chunk(s).
    pub create_chunk_call: SharedSemaphore,

    // The create_chunk_wait semaphore limits the number of simultaneous
    // agent.wait() calls for outstanding create_chunk(s) requests.
    pub create_chunk_wait: SharedSemaphore,
}

impl Semaphores {
    pub fn new() -> Semaphores {
        let file = SharedSemaphore::new(true, MAX_SIMULTANEOUS_LOADED_MB);

        let create_chunk = SharedSemaphore::new(true, MAX_SIMULTANEOUS_CREATE_CHUNK);

        let create_chunk_call = SharedSemaphore::new(true, MAX_SIMULTANEOUS_CREATE_CHUNK_CALLS);

        let create_chunk_wait = SharedSemaphore::new(true, MAX_SIMULTANEOUS_CREATE_CHUNK_WAITS);

        Semaphores {
            file,
            create_chunk,
            create_chunk_call,
            create_chunk_wait,
        }
    }
}
