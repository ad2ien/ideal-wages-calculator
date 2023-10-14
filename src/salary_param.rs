use serde::Deserialize;

const _PARAM_DATA_URL: &str = "http://localhost:1984/params";

#[derive(Clone, Debug, Deserialize)]
pub struct SalaryParam {
    pub id: &'static str,
    pub value: i8,
}

impl PartialEq for SalaryParam {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
