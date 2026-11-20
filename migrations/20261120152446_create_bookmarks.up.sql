-- One row per bookmark. Column names and types line up with the
-- Bookmark struct in src/models.rs (id as i64, the rest as String).
CREATE TABLE bookmarks (
    id    BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT NOT NULL,
    url   TEXT NOT NULL,
    tag   TEXT NOT NULL
);

-- Speeds up GET /bookmarks?tag=...
CREATE INDEX bookmarks_tag_idx ON bookmarks (tag);

-- The two example bookmarks the in-memory store started with.
-- No id given, so Postgres assigns 1 and 2.
INSERT INTO bookmarks (title, url, tag) VALUES
    ('Example',         'https://example.com', 'A'),
    ('Another Example', 'https://another.com', 'B');
