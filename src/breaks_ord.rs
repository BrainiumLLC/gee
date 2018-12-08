/// Trait for identifying values that break a total ordering for that type.
///
/// - returns false if the type is not `PartialOrd`.
/// - `NaN` returns `true`.
/// - +/- infinity return `true` because multiple values clamp to infinity.
pub unsafe trait BreaksOrd {
    fn breaks_ord(&self) -> bool;
}

unsafe impl<T: ?Sized> BreaksOrd for T {
    #[inline]
    default fn breaks_ord(&self) -> bool {
        self.breaks_ord_spec()
    }
}

unsafe impl BreaksOrd for f32 {
    #[inline]
    fn breaks_ord(&self) -> bool {
        !self.is_finite()
    }
}

unsafe impl BreaksOrd for f64 {
    #[inline]
    fn breaks_ord(&self) -> bool {
        !self.is_finite()
    }
}

unsafe trait BreaksOrdSpec {
    fn breaks_ord_spec(&self) -> bool;
}

unsafe impl<T: ?Sized> BreaksOrdSpec for T {
    #[inline]
    default fn breaks_ord_spec(&self) -> bool {
        // if a type is not even PartialOrd then values of it can't break Ord
        false
    }
}

unsafe impl<T: ?Sized + PartialOrd> BreaksOrdSpec for T {
    #[inline]
    default fn breaks_ord_spec(&self) -> bool {
        // assuming ordering is broken for all values of types that are PartialOrd but not Ord
        // to override, specialize BreaksOrd
        true
    }
}

unsafe impl<T: ?Sized + Ord> BreaksOrdSpec for T {
    #[inline]
    fn breaks_ord_spec(&self) -> bool {
        false
    }
}
