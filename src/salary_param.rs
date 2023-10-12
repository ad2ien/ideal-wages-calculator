#[derive(Copy, Clone, Debug)]
pub struct SalaryParam {
    pub id: &'static str,
    pub label: &'static str,
    pub coefficient: f64,
    pub value: i8,
}

impl PartialEq for SalaryParam {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
