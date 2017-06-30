
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc<T, P> {
    pub inner: T,
    pub pos: P,
}

impl<T, P> ::std::ops::Deref for Loc<T, P> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, P> ::std::fmt::Display for Loc<T, P>
    where T: ::std::fmt::Display
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T, P> ::std::fmt::Debug for Loc<T, P>
    where T: ::std::fmt::Debug,
          P: ::std::fmt::Debug
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "<{:?}@{:?}>", self.inner, self.pos)
    }
}