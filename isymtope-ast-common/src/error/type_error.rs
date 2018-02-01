use std::fmt;

#[derive(Debug)]
pub enum DocumentTypeError {
    TypeError(String),
    // MismatchActionParam(String, String, VarType, VarType)
}

// // impl DocumentTypeError {
//     pub fn mismatch_action_param(complete_key: &str, param_key: &str, existing_ty: &VarType, ty: &VarType) -> DocumentTypeError {
//         DocumentTypeError::MismatchActionParam(complete_key.to_owned(), param_key.to_owned(), existing_ty.to_owned(), ty.to_owned())
//     }
// }

impl Error for DocumentTypeError {
    fn description(&self) -> &str {
        match self {
            &DocumentTypeError::TypeError(..) => "Type error in document",
            // &DocumentTypeError::MismatchActionParam(..) => "Type error in document: reducer action param has different type than previous dispatch of this action."
        }
    }
}

impl Display for DocumentTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DocumentTypeError::TypeError(ref e) => e.fmt(f),
            // &DocumentTypeError::MismatchActionParam(ref complete_key, ref param_key, ref previous_ty, ref new_ty) => {
            //     write!(f, "Type error: reducer action ({}) has existing type for param ({}) of ({:?}), attempting to dispatch the action again with a different type ({:?}) for this parameter.",
            //         complete_key, param_key, previous_ty, new_ty)
            // }
        }
    }
}
