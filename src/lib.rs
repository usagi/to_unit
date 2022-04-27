/// Any type -> ()
pub trait ToUnit
{
 /// Any type -> ()
 fn to_unit(&self) {}
}

/// Any type -> ()
impl<T> ToUnit for T {}

/// Any type -> ()
/// Same to `to_unit::ToUnit`, but it's type more easily.
pub trait __
{
 /// Any type -> ()
 fn __(&self) {}
}

/// Any type -> ()
impl<T> __ for T {}
