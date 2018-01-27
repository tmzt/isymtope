

pub trait AsValue<T>: Sized {
    fn as_value(&self) -> Option<T>;
}

pub trait ValueFrom<T>: Sized {
    fn value_from(&T) -> Option<Self>;
}

pub trait MaybeRef<T> {
    fn maybe_ref(&self) -> Option<&T>;
}

impl<T, U> AsValue<U> for T where U: ValueFrom<T> {
    default fn as_value(&self) -> Option<U> {
        U::value_from(self)
    }
}

pub trait AsPair<'a> {
    type V;
    
    fn as_pair(&'a self) -> (&'a str, Option<&'a Self::V>);
}

// impl<Src, Dest, Via> ValueFrom<Src> for Dest where Via: ValueFrom<Src>, Dest: ValueFrom<Via> {
//     fn value_from(src: &Src) -> Option<Dest> {
//         Via::value_from(src).and_then(|v| Dest::value_from(v))
//     }
// }

// impl<Src: Sized, Dest: Sized, Via: Sized> ValueFrom<Src> for Dest where Dest: ValueFrom<Via>, Via: ValueFrom<Src> {
//     fn value_from(src: &Src) -> Option<Dest> {
//         let via = Via::value_from(src);
//         via.and_then(|via| Dest::value_from(via))
//     }
// }

// pub trait AsReducedExpr {
//     fn as_reduced_expr(&self) -> Option<ReducedExpr>;
// }

// pub trait AsSourceExpr {
//     fn as_source_expr(&self) -> Option<SourceExpr>;
// }

// pub trait AsConstExpr {
//     fn as_const_expr(&self) -> Option<ConstExpr>;
// }

// pub trait AsPrimitiveConstExpr {
//     fn as_prim_const_expr(&self) -> Option<ConstExpr>;
// }