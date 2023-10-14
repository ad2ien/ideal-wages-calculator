use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Criteria {
    pub id: &'static str,
    pub label: &'static str,
    pub coefficient: f64,
    pub description: &'static str,
}

impl PartialEq for Criteria {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


pub const CRITERIAS: [Criteria; 7] = [
    Criteria {
        id: "hate",
        label: "How much I hate what I'm doing?",
        coefficient: 1.0,
        description: "0: you hate everything you're doing, 10: you save the world... and you think the world is worth saving",
    },
    Criteria {
        id: "body",
        label: "How much I destroy my body at work?",
        coefficient: 1.0,
        description: "How much I destroy my body at work?",
    },
    Criteria {
        id: "pain",
        label: "How much pain I get from doing my job?",
        coefficient: 1.0,
        description: "How much pain I get from doing my job?",
    },
    Criteria {
        id: "mental",
        label: "How much I'm emotionally impacted by my work?",
        coefficient: 1.0,
        description: "How much I'm emotionally impacted by my work?",
    },
    Criteria {
        id: "value",
        label: "How much value I bring with my work?",
        coefficient: 1.0,
        description: "How much value I bring with my work?",
    },
    Criteria {
        id: "skills",
        label: "How rare are my skills?",
        coefficient: 1.0,
        description: "How rare are my skills?",
    },
    Criteria {
        id: "training",
        label: "How much I sacrifice to train my skills?",
        coefficient: 1.0,
        description: "How much I sacrifice to train my skills?",
    },
];
