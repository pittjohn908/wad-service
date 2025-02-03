CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255),
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    date_created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (email)
);

CREATE TABLE auth_providers (
    id SERIAL PRIMARY KEY,
    provider_name VARCHAR(50),
    UNIQUE (provider_name)
);
INSERT INTO auth_providers (provider_name) VALUES
('local'),
('apple'),
('google'),
('facebook');

CREATE TABLE user_authentication (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    auth_provider_id INTEGER REFERENCES auth_providers (
        id
    ) ON DELETE CASCADE NOT NULL,
    refresh_token TEXT NOT NULL,
    token_expiry TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE (user_id, auth_provider_id)
);

CREATE TABLE auth_local (
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    password_hash TEXT NOT NULL,
    last_login TIMESTAMP WITH TIME ZONE,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (user_id)
);


CREATE INDEX idx_user_emails ON users (email);

-- Create function to hash password using SCRAM-SHA-256
CREATE OR REPLACE FUNCTION hash_password (password TEXT)
RETURNS TEXT AS $$
BEGIN
    RETURN crypt(password, gen_salt('bf'));
END;
$$
LANGUAGE plpgsql SECURITY DEFINER ;

-- Create function to verify password
CREATE OR REPLACE FUNCTION verify_password (stored_hash TEXT,
input_password TEXT)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN stored_hash = crypt(input_password, stored_hash);
END;
$$
LANGUAGE plpgsql SECURITY DEFINER ;
