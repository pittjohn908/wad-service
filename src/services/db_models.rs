use crate::grpc::{DefinitionDetails, WordDetails};

#[derive(Debug, Default, sqlx::FromRow)]
pub(crate) struct DbWord {
    pub id: i32,
    pub text: String,
    pub language: String,
    pub difficulty: i32,
}

#[derive(Debug, Default, sqlx::FromRow)]
pub(crate) struct DbDefinition {
    pub id: i32,
    pub word_id: i32,
    pub meaning: String,
    pub part_of_speech: String,
    pub usage: String
}

impl WordDetails {
    pub(crate) fn from(db_word: DbWord, db_definitions: Vec<DbDefinition>) -> Self {
        Self {
            text: db_word.text,
            language: db_word.language,
            definitions: db_definitions.into_iter().map(DefinitionDetails::from).collect(),
        }
    }
}

impl DefinitionDetails {
    pub(crate) fn from(db_definition: DbDefinition) -> Self {
        Self {
            meaning: db_definition.meaning,
            part_of_speech: db_definition.part_of_speech,
            usage: db_definition.usage,
        }
    }
}