use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Criteria {
    pub id: String,
    pub label: String,
    pub coefficient: f64,
    pub description: CriteriaDescription,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CriteriaDescription {
    pub general: String,
    pub min: String,
    pub max: String,
}

impl PartialEq for Criteria {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
