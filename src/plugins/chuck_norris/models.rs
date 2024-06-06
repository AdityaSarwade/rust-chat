use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RandomJokeResult {
    // pub categories: Vec<String>,
    // pub created_at: String,
    // pub icon_url: String,
    // pub id: String,
    // pub updated_at: String
    // pub url: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoriesResult(pub Vec<String>);
