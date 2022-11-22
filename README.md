# effortless

## description
- An effort for effortless learning.

## hard earned wisdom
- The best task management system until now is a white board and a marker.
- The best information note taking system is taking no notes at all.
- The best running notes are the ones that are forgotten shortly later.
- The best information gathering system until now is this.
- Learning by example and by answering questions is good.
- Unstructured might be good.
- School bag, escape errands v1 v2, python+latex journal, rust rocket journal, next zola blog with examples, MCQs, interactive wasms?

## usage
- Install `nightly` version `rust`.
- Use `cargo +nightly run --release` to start the server.
- The server can then (generally be) accessed at `http://127.0.0.1:8000`.
- The first few pages of the site explains the purpose, structure, ... The later pages contain some functionality.
- Currently the main features are,
    - Scratch page.
    - DAG of statements.
- Functionality should be self-explaining.
- There is not too rigourous testing or security check, though the `rust` and `rocket` takes care of most of it.
    - To prevent unwanted data loss, make checkpoints of `db/` content (checkpoint files will not be touched by server) and back them up regularly in cloud (preferably github).
