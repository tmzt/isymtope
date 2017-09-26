
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use processing::*;


pub type DefaultStateMap = LinkedHashMap<String, (Option<VarType>, Option<ExprValue>)>;

pub type ReducerKeyMap = HashMap<String, ReducerKeyData>;
pub type ReducerKeyProcessingMap = HashMap<String, ReducerKeyProcessing>;
pub type ReducerActionTypeMap = HashMap<String, PropTypeMap>;

#[derive(Debug, Default, PartialEq)]
pub struct ReducerKeyProcessing {
    pub reducer_key: String,
    pub default_expr: Option<ExprValue>,
    pub ty: Option<VarType>,
    pub actions: Vec<ReducerActionProcessing>,
    pub action_tys: ReducerActionTypeMap
}

// impl ReducerKeyProcessing {
//     pub fn insert_prop_type(&mut self, complete_key: &str, param_key: &str, ty: &VarType) -> Result {
//         let action_ty_data = self.action_tys.entry(complete_key.to_owned()).or_insert(Default::default());

//         match action_ty_data.entry(param_key.to_owned()) {
//             Entry::Occupied(o) => {
//                 if let &Some(ref existing_ty) = o.get() {
//                     if existing_ty != ty {
//                         return Err(DocumentTypeError::mismatch_action_param(complete_key, param_key, &existing_ty, ty).into());
//                     };
//                 };
//             },
//             Entry::Vacant(v) => {
//                 v.insert(Some(ty.to_owned()));
//             }
//         };
//         Ok(())
//     }
// }

impl ReducerKeyProcessing {
    pub fn from_name(reducer_key: &str, ty: Option<VarType>) -> Self {
        let reducer_key = reducer_key.to_owned();

        ReducerKeyProcessing {
            reducer_key: reducer_key,
            default_expr: None,
            ty: ty,
            actions: Default::default(),
            action_tys: Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ReducerKeyData {
    pub reducer_key: String,
    pub default_expr: Option<ExprValue>,
    pub ty: Option<VarType>,
    pub action_tys: Option<ReducerActionTypeMap>,
    pub actions: Option<Vec<ReducerActionData>>
}

impl Into<ReducerKeyData> for ReducerKeyProcessing {
    fn into(self) -> ReducerKeyData {
        let action_tys = if self.action_tys.is_empty() { None } else { Some(self.action_tys) };
        let actions: Vec<ReducerActionData> = self.actions.into_iter().map(|act| act.into()).collect();
        let actions = if actions.is_empty() { None } else { Some(actions) };

        ReducerKeyData {
            reducer_key: self.reducer_key,
            default_expr: self.default_expr,
            ty: self.ty,
            action_tys: action_tys,
            actions: actions
        }
    }
}

impl ReducerKeyData {
    #[allow(dead_code)]
    pub fn actions<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ReducerActionData>> {
        self.actions.as_ref().map(|v| v.into_iter())
    }
}

#[derive(Debug, PartialEq)]
pub struct ReducerActionProcessing {
    pub action_type: String,
    pub state_expr: Option<ActionStateExprType>,
    pub state_ty: Option<VarType>,
    pub default_scope_key: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ReducerActionData {
    pub action_type: String,
    pub state_expr: Option<ActionStateExprType>,
    pub state_ty: Option<VarType>,
    pub default_scope_key: Option<String>,
}

impl Into<ReducerActionData> for ReducerActionProcessing {
    fn into(self) -> ReducerActionData {
        ReducerActionData {
            action_type: self.action_type,
            state_expr: self.state_expr,
            state_ty: self.state_ty,
            default_scope_key: self.default_scope_key
        }
    }
}

impl ReducerActionProcessing {
    pub fn from_name(action_name: &str, default_scope_key: Option<&str>) -> Self {
        let action_type = action_name.to_uppercase();
        let default_scope_key = default_scope_key.map(String::from);

        ReducerActionProcessing {
            action_type: String::from(action_type),
            state_expr: None,
            state_ty: None,
            default_scope_key: default_scope_key,
        }
    }
}