CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS preview (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    pk SERIAL,
    link TEXT NOT NULL,
    --
    name VARCHAR(255) NOT NULL,
    description TEXT,
    keywords TEXT,
    image TEXT,
    --
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_preview_id ON preview (id);
CREATE UNIQUE INDEX idx_preview_pk ON preview (pk);
CREATE UNIQUE INDEX idx_preview_link ON preview (link);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON preview
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at_column();
