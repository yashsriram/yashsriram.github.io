# effortless

## description
- An effort for effortless learning.

## code
- Code is written in `rust`.
- Uses `rocket` library to create a server.
- `db/` contains all the content that is created.
- `src/` contains the source code.
- `static/` contains static files.
- `templates/` contains the templates.

## documentation
- The documentation of the code is itself.

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

## roadmap
- [ ] Simplify goal, design.
    - [x] Improve name.
    - [x] Use only axioms and theorems, remove definitions.
    - [x] Use only statements, remove axioms and theorems.
    - Text representation.
        - [x] Case combination check.
        - [x] Every statement has a family tree. One can traverse using inline links to its parent statements.
    - [x] Use stripped snake case for ids
    - [ ] Scratch
        - [ ] Port todos and scratch from journal to scratch page.
        - [ ] Saving versions.
        - [ ] Solve Todos in journal.
        - [ ] First class TODOs, partials support.
        - [ ] Handle TODOs and scratch statements.
-  Make sure that design minimizes constructs. Minimize current content.
    - [ ] Typed journal.
    - [ ] More terse text.
    - [ ] Other topological sorts.
    - [ ] Measure of clumsiness and cleaning the graph. Detect clumsy additions on create. Suggest cleaning graph.
    - [ ] Design should be such that clumsy graph should be difficult to create.
    - [ ] Remove unnecessary redundancies. For example, in `Let P(A) denote @probability@ of some @event@ A of a @stochastic experiment@ E` probability already is defined for an event and experiment. Should not have to declare it again.
    - [ ] Solve problem: For every new statement, I have to traverse entire graph and check for duplicates, optimize graph ...
- Build a Server Client for browsing.
    - [ ] CRUD of statements.
    - [ ] Colors.
    - [x] Left right to move across statements. Instead of that prev and next links at relatively same positions.
    - [x] Statement id list view/Statement complete view.
    - [x] Able to search in entire graph.
    - [x] Followable links to parents and children.
    - [x] Saving version.
    - [ ] Uploading to github.
    - [ ] Chapter seperators.
- Graph representation.
    - [x] Graph page.
    - [x] DAG node mass caliberation.
    - [x] DAG node size caliberation.
    - [x] DAG statement heatmap.
    - [x] btn to reset.
    - [x] Click to go to statement.
    - [ ] DAG search.
    - [ ] DAG latex type set description.
    - [ ] More control on rendering and ui (need force directed layout implementation i.e. spring-mass simulation).
