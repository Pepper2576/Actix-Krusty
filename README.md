# krusty

A small CRUD API for bookmarks, built with [actix-web](https://actix.rs/).

Bookmarks are stored in memory, so they reset whenever the server restarts. The server starts with two example bookmarks.

## Requirements

- Rust 1.85 or newer (the crate uses the 2024 edition)

## Running

```sh
cargo run
```

The server listens on `0.0.0.0:8080` by default. To use a different port, set the `PORT` environment variable:

```sh
# bash / zsh
PORT=3000 cargo run

# PowerShell
$env:PORT = "3000"; cargo run
```

The `.env` file is not loaded automatically, so `PORT` has to be set in your shell.

## Bookmark shape

```json
{
  "id": 1,
  "title": "Example",
  "url": "https://example.com",
  "tag": "A"
}
```

`id` is assigned by the server. Create (`POST`) and replace (`PUT`) requests must send all three of `title`, `url` and `tag`. Partial update (`PATCH`) requests can send any of those fields, and fields left out stay as they are.

## Endpoints

| Method   | Path              | Description                              | Success response         |
| -------- | ----------------- | ---------------------------------------- | ------------------------ |
| `GET`    | `/`               | Hello message                            | `200` text               |
| `GET`    | `/about`          | About message                            | `200` text               |
| `GET`    | `/health`         | Health check                             | `200` `OK`               |
| `GET`    | `/bookmarks`      | List bookmarks, optionally by tag        | `200` JSON array         |
| `GET`    | `/bookmarks/{id}` | Get one bookmark                         | `200` JSON               |
| `POST`   | `/bookmarks`      | Create a bookmark                        | `201` JSON               |
| `PUT`    | `/bookmarks/{id}` | Replace a bookmark's title, URL and tag  | `200` JSON               |
| `PATCH`  | `/bookmarks/{id}` | Update only the fields sent              | `200` JSON               |
| `DELETE` | `/bookmarks/{id}` | Delete a bookmark                        | `200` `Bookmark deleted` |

If no bookmark has the given `{id}`, the endpoint returns `404` with the body `Bookmark not found`.

## Examples

```sh
# List all bookmarks
curl http://localhost:8080/bookmarks

# List bookmarks with tag "A"
curl "http://localhost:8080/bookmarks?tag=A"

# Get one bookmark
curl http://localhost:8080/bookmarks/1

# Create a bookmark
curl -X POST http://localhost:8080/bookmarks \
  -H "Content-Type: application/json" \
  -d '{"title": "Rust", "url": "https://www.rust-lang.org", "tag": "dev"}'

# Replace a bookmark (all fields required)
curl -X PUT http://localhost:8080/bookmarks/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Updated", "url": "https://example.org", "tag": "B"}'

# Change just the tag (other fields unchanged)
curl -X PATCH http://localhost:8080/bookmarks/1 \
  -H "Content-Type: application/json" \
  -d '{"tag": "reading"}'

# Delete a bookmark
curl -X DELETE http://localhost:8080/bookmarks/1
```

In Windows PowerShell, use `curl.exe` instead of `curl`, because `curl` is an alias for `Invoke-WebRequest` there.

## Project layout

```
src/
├── main.rs            # Server setup and route registration
├── models.rs          # Bookmark and request types
├── state.rs           # Shared in-memory store (Mutex<Vec<Bookmark>>)
└── handlers/
    ├── general.rs     # /, /about, /health
    └── bookmarks.rs   # /bookmarks CRUD handlers
```

## Limitations

- Data is not persisted. Restarting the server resets it to the two example bookmarks.
- A new bookmark's ID is the last bookmark's ID plus one. If you delete the newest bookmark, its ID is given to the next one you create.
- URLs and other fields are not validated.
