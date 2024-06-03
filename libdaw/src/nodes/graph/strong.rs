use crate::Node;
use nohash_hasher::IsEnabled;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

/// A wrapper for a dynamic node that is comparable and hashable by pointer.
#[derive(Debug)]
pub struct Strong {
    /// The strong node
    pub inner: Arc<Mutex<dyn Node>>,
}

impl Clone for Strong {
    fn clone(&self) -> Self {
        Strong {
            inner: self.inner.clone(),
        }
    }
}

impl IsEnabled for Strong {}

impl DerefMut for Strong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for Strong {
    type Target = Arc<Mutex<dyn Node>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for Strong {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Strong {}

impl PartialOrd for Strong {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_: *const () = Arc::as_ptr(&self.inner).cast();
        let other: *const () = Arc::as_ptr(&other.inner).cast();
        self_.partial_cmp(&other)
    }
}

impl Ord for Strong {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_: *const () = Arc::as_ptr(&self.inner).cast();
        let other: *const () = Arc::as_ptr(&other.inner).cast();
        self_.cmp(&other)
    }
}

impl Hash for Strong {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let self_: *const () = Arc::as_ptr(&self.inner).cast();
        state.write_usize(self_ as usize)
    }
}
// /// A wrapper for a dynamic node that is comparable and hashable by pointer.
// #[derive(Debug, Clone)]
// pub struct Weak {
//     weak: WeakArc<Mutex<dyn Node>>,
// }

// impl IsEnabled for Weak {}

// impl DerefMut for Weak {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.weak
//     }
// }

// impl Deref for Weak {
//     type Target = WeakArc<Mutex<dyn Node>>;

//     fn deref(&self) -> &Self::Target {
//         &self.weak
//     }
// }

// impl PartialEq for Weak {
//     fn eq(&self, other: &Self) -> bool {
//         WeakArc::ptr_eq(&self.weak, &other.weak)
//     }
// }

// impl Eq for Weak {}

// impl PartialOrd for Weak {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         let self_: *const () = WeakArc::as_ptr(&self.weak).cast();
//         let other: *const () = WeakArc::as_ptr(&other.weak).cast();
//         self_.partial_cmp(&other)
//     }
// }

// impl Ord for Weak {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let self_: *const () = WeakArc::as_ptr(&self.weak).cast();
//         let other: *const () = WeakArc::as_ptr(&other.weak).cast();
//         self_.cmp(&other)
//     }
// }

// impl Hash for Weak {
//     fn hash<H>(&self, state: &mut H)
//
//         H: Hasher,
//     {
//         let self_: *const () = WeakArc::as_ptr(&self.weak).cast();
//         state.write_usize(self_ as usize)
//     }
// }

// impl PartialEq<Weak> for Strong {
//     fn eq(&self, other: &Weak) -> bool {
//         WeakArc::ptr_eq(&self.weak.weak, &other.weak)
//     }
// }
// impl PartialEq<Strong> for Weak {
//     fn eq(&self, other: &Strong) -> bool {
//         WeakArc::ptr_eq(&self.weak, &other.weak.weak)
//     }
// }
// impl PartialOrd<Weak> for Strong {
//     fn partial_cmp(&self, other: &Weak) -> Option<std::cmp::Ordering> {
//         let self_: *const () = Arc::as_ptr(&self.strong).cast();
//         let other: *const () = WeakArc::as_ptr(&other.weak).cast();
//         self_.partial_cmp(&other)
//     }
// }
// impl PartialOrd<Strong> for Weak {
//     fn partial_cmp(&self, other: &Strong) -> Option<std::cmp::Ordering> {
//         let self_: *const () = WeakArc::as_ptr(&self.weak).cast();
//         let other: *const () = Arc::as_ptr(&other.strong).cast();
//         self_.partial_cmp(&other)
//     }
// }

// impl Borrow<Weak> for Strong {
//     fn borrow(&self) -> &Weak {
//         &self.weak
//     }
// }
