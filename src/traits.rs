use crate::ContextHandle;
use geos_sys::GEOSContextHandle_t;

pub trait AsRaw {
    type RawType;

    fn as_raw(&self) -> *const Self::RawType;
}

pub trait AsRawMut {
    type RawType;

    fn as_raw_mut(&mut self) -> *mut Self::RawType {
        unsafe { self.as_raw_mut_override() }
    }
    /// This method exists because in certain case, even though you don't run any mutable operation
    /// on the object, it still requires a mutable access.
    ///
    /// A good example is `GEOSWKTWriter_getOutputDimension_r` (which is very likely a bug).
    unsafe fn as_raw_mut_override(&self) -> *mut Self::RawType;
}

pub trait ContextHandling {
    type Context;

    #[doc(hidden)]
    fn get_raw_context(&self) -> GEOSContextHandle_t;
    fn clone_context(&self) -> Self::Context;
}

pub trait ContextInteractions {
    fn set_context_handle(&mut self, context: ContextHandle);
    fn get_context_handle(&self) -> &ContextHandle;

    /// Gets the last error (if any) from the [`ContextHandle`] held by this object.
    ///
    /// # Example
    ///
    /// ```
    /// use geos::{ContextInteractions, Geometry};
    ///
    /// let mut point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// // execute some calls on `point_geom`
    /// point_geom.get_last_error();
    /// // This is a shortcut for calling:
    /// point_geom.get_context_handle().get_last_error();
    /// ```
    fn get_last_error(&self) -> Option<String> {
        self.get_context_handle().get_last_error()
    }

    /// Gets the last notification (if any) from the [`ContextHandle`] held by this object.
    ///
    /// # Example
    ///
    /// ```
    /// use geos::{ContextInteractions, Geometry};
    ///
    /// let mut point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// // execute some calls on `point_geom`
    /// point_geom.get_last_notification();
    /// // This is a shortcut for calling:
    /// point_geom.get_context_handle().get_last_notification();
    /// ```
    fn get_last_notification(&self) -> Option<String> {
        self.get_context_handle().get_last_notification()
    }
}
