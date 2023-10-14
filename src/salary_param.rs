#[derive(Copy, Clone, Debug)]
pub struct SalaryParam {
    pub id: &'static str,
    pub value: i8,
}

impl PartialEq for SalaryParam {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
