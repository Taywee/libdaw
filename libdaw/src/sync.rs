use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Default)]
pub struct AtomicF64 {
    storage: AtomicU64,
}

impl fmt::Debug for AtomicF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load(Ordering::Relaxed))
    }
}

impl AtomicF64 {
    pub fn new(value: f64) -> Self {
        Self::from(value)
    }
    pub fn store(&self, val: f64, order: Ordering) {
        self.storage.store(val.to_bits(), order)
    }
    pub fn load(&self, order: Ordering) -> f64 {
        f64::from_bits(self.storage.load(order))
    }
    pub fn swap(&self, val: f64, order: Ordering) -> f64 {
        f64::from_bits(self.storage.swap(val.to_bits(), order))
    }
    #[allow(dead_code)]
    pub fn compare_exchange(
        &self,
        current: f64,
        new: f64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f64, f64> {
        self.storage
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(f64::from_bits)
            .map_err(f64::from_bits)
    }
    #[allow(dead_code)]
    pub fn compare_exchange_weak(
        &self,
        current: f64,
        new: f64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f64, f64> {
        self.storage
            .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
            .map(f64::from_bits)
            .map_err(f64::from_bits)
    }
}

impl From<f64> for AtomicF64 {
    fn from(value: f64) -> Self {
        Self {
            storage: AtomicU64::new(value.to_bits()),
        }
    }
}
