use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WagesParam {
    pub id: String,
    pub value: i8,
}

impl PartialEq for WagesParam {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
