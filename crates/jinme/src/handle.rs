use crate::prelude::*;
use ::std::{
    cmp, fmt,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};
use as_any::{AsAny, Downcast};

mod buf_read_handle;
mod write_handle;

pub use buf_read_handle::BufReadHandle;
pub use write_handle::WriteHandle;

/// Trait for handle implementations that can be downcast to specific types.
///
/// This trait enables polymorphic handling of different handle types while
/// maintaining type safety through the `as_any` crate.
///
/// Implementations must be `Send` and `Sync` for thread safety.
pub trait IHandle: Send + Sync + AsAny {}

/// A handle for external resources with downcast support.
///
/// The `Handle` type wraps a `Mutex<dyn IHandle>` to provide thread-safe access
/// to polymorphic handle implementations. It supports downcasting to specific
/// handle types for type-specific operations.
///
/// # Thread Safety
///
/// The handle uses `Mutex` to protect access to the inner handle implementation,
/// making it safe for concurrent access from multiple threads.
///
/// # Downcasting
///
/// Handles can be downcast to specific types using the `downcast_ref` and
/// `with_downcast_ref` methods.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::io::Cursor;
/// let handle = BufReadHandle::new(Cursor::new("hello".to_string()));
/// let handle = Handle::new(handle);
///
/// // Downcast to specific type
/// if let Some(text) = handle.downcast_ref::<BufReadHandle>() {
///     let _ = text;
/// }
/// ```
#[derive(Clone)]
pub struct Handle(Arc<Mutex<dyn IHandle>>);

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Handle {}

impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Arc::as_ptr(&self.0)
            .cast::<()>()
            .partial_cmp(&Arc::as_ptr(&other.0).cast::<()>())
    }
}

impl Ord for Handle {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Arc::as_ptr(&self.0)
            .cast::<()>()
            .cmp(&Arc::as_ptr(&other.0).cast::<()>())
    }
}

impl Hash for Handle {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        // TODO: a real hash impl, this is likely invalid and unsafe
        Arc::as_ptr(&self.0).cast::<()>().hash(hasher);
    }
}

impl Handle {
    /// Apply a function to a downcast reference of type T, if the inner value is T.
    ///
    /// # Arguments
    ///
    /// * `f` - Function to apply to the downcasted reference
    ///
    /// # Returns
    ///
    /// `Some(result)` if the handle can be downcast to T, `None` otherwise
    pub fn with_downcast_ref<T, F, R>(&self, f: F) -> Option<R>
    where
        T: IHandle + 'static,
        F: FnOnce(&T) -> R,
    {
        if let Ok(guard) = self.0.lock() {
            Downcast::downcast_ref::<T>(&*guard).map(f)
        } else {
            None
        }
    }

    /// Try to get a cloned reference of type T, if T is Clone and the inner value is T.
    ///
    /// # Returns
    ///
    /// `Some(cloned_value)` if the handle can be downcast to T, `None` otherwise
    pub fn downcast_ref<T>(&'_ self) -> Option<T>
    where
        T: IHandle + Clone + 'static,
    {
        if let Ok(guard) = self.0.lock() {
            Downcast::downcast_ref::<T>(&*guard).cloned()
        } else {
            None
        }
    }

    /// Try to mutate the inner value as type T using a closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Function to apply to the mutable downcasted reference
    ///
    /// # Returns
    ///
    /// `Some(())` if the handle can be downcast to T, `None` otherwise
    pub fn downcast_mut<T, F>(&self, f: F) -> Option<()>
    where
        T: IHandle + 'static,
        F: FnOnce(&mut T),
    {
        if let Ok(mut guard) = self.0.lock() {
            if let Some(t) = Downcast::downcast_mut::<T>(&mut *guard) {
                f(t);
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Handle {
    /// Creates a new Handle from an IHandle implementation.
    ///
    /// # Arguments
    ///
    /// * `value` - The IHandle implementation to wrap
    ///
    /// # Returns
    ///
    /// A new Handle wrapping the implementation
    pub fn new<T: IHandle>(value: T) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addr = Arc::as_ptr(&self.0).cast::<()>();
        if let Some(ns) = self.downcast_ref::<PtrNamespace>() {
            write!(
                f,
                "#handle[jinme.Namespace {:p} \"{}\"]",
                addr,
                ns.name_str()
            )
        } else if let Some(_) = self.downcast_ref::<BufReadHandle>() {
            write!(f, "#handle[jinme.BufReadHandle {:p}]", addr)
        } else if let Some(_) = self.downcast_ref::<WriteHandle>() {
            write!(f, "#handle[jinme.WriteHandle {:p}]", addr)
        } else if let Some(func) = self.downcast_ref::<PtrFunction>() {
            write!(
                f,
                "#handle[jinme.Function {:p} {}]",
                addr,
                func.name().unwrap_or("<unnamed>")
            )
        } else {
            write!(f, "#handle[{:p}]", addr)
        }
    }
}
