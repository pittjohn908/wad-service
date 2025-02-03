CREATE TABLE words (
    id SERIAL PRIMARY KEY,
    text VARCHAR(255) NOT NULL,
    language VARCHAR(255) NOT NULL,
    difficulty INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE definitions (
    id SERIAL PRIMARY KEY,
    word_id INTEGER REFERENCES words (id) ON DELETE CASCADE NOT NULL,
    meaning TEXT NOT NULL,
    part_of_speech TEXT NOT NULL,
    usage TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_favorites (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    word_id INTEGER REFERENCES words (id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, word_id)
);

CREATE INDEX idx_words_text ON words (text);
CREATE INDEX idx_words_language ON words (language);
CREATE INDEX idx_definitions_word_id ON definitions (word_id);
CREATE INDEX idx_user_favorites_user_id ON user_favorites (user_id);
