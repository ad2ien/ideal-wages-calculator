use serde::Deserialize;
use crate::wages_param::WagesParam;


#[derive(Clone, Debug, Deserialize)]
pub struct Job {
    pub job: String,
    pub params: Vec<WagesParam>,
}