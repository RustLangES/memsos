use heapless::{Vec, String};
use core::sync::atomic::{AtomicUsize, Ordering};

const VEC_SIZE: usize = 128;
const STRING_SIZE: usize = 1028;

pub struct Ask {
    pub options: Vec<String<STRING_SIZE>, VEC_SIZE>,
    current: AtomicUsize,
}



