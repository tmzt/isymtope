use uuid::Uuid;

pub fn allocate_element_key() -> String {
    let uuid_ = format!("{}", Uuid::new_v4());
    format!("{0:.3}", uuid_)
}
