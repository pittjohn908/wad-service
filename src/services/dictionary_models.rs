
#[derive(Debug, Default, sqlx::FromRow)]
pub(crate) struct DbWord {
    pub id: i64,
    pub text: String,
    pub language: String,
    pub difficualty: i32,
}

#[derive(Debug, Default, sqlx::FromRow)]
pub(crate) struct DbDefinition {
    pub id: i64,
    pub word_id: i64,
    pub meaning: String,
    pub part_of_speech: String,
    pub usage: String
}