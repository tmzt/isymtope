use uuid::Uuid;

pub fn allocate_element_key() -> String {
    let uuid_ = Uuid::new_v4();
    format!("{:8}", uuid_)
}