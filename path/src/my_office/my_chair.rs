
pub struct Chair {
    pub name: String,
    pub message: String,
}

#[allow(clippy::new_without_default)]
impl Chair {
    pub fn new() -> Chair {
        Chair {
            name: String::from("Chair"),
            message: String::from("Sitting!"),
        }
    }
}

pub fn sit() -> Chair {
    Chair::new()
}