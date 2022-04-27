pub trait ToUnit
{
 fn to_unit(&self) {}
}

impl<T> ToUnit for T {}
