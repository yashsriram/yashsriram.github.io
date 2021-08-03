# effortless

## description
- An effort for effortless learning.

## code
- Code is written in `rust`.
- Uses `rocket` library to create a server.
- `db/` contains all the content that is created.
    - `db/latest.md` and `db/latest.json` are used for scratch page and DAG statements respectively.
- `src/` contains the source code.
- `static/` contains static files.
- `templates/` contains the templates.
- This was first implemented in `python`. That source code and data can be found in the repo history at commit `a50970a2951bd46f4fc46c3c082a01f1f88df243`.

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

## demonstration

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
        - [x] Handle TODOs and scratch statements.
        - [x] First class TODOs, partials support.
        - [x] Port todos and scratch from journal to scratch page.
        - [x] Making checkpoints.
        - [ ] Update scratch.
- Build a Server Client for browsing.
    - [ ] CRUD of statements.
        - [x] R
        - [x] C
        - [ ] U
        - [ ] D
    - [ ] Open page navigations.
        - [x] URL.
        - [x] List.
        - [x] Parents/Children.
        - [ ] Show ancestors and descendants graphs, which can open statements.
    - [ ] Colors.
    - [ ] Chapter seperators.
    - [ ] Uploading to github.
        - [ ] The intended use of this system is to start server on localhost and use browser/client as a UI. Kind of like a native application. Not meant to be deployed on server.
        - [ ] This way any additions/updates to content or code are nicely commited and pushed to gituhub.
        - [ ] On any critical bug which erases content, you can get latest content from github.
        - [ ] That is why it is important to commit regularly.
    - [x] Left right to move across statements. Instead of that prev and next links at relatively same positions.
    - [x] Statement id list view/Statement complete view.
    - [x] Able to search in entire graph.
    - [x] Followable links to parents and children.
    - [x] Making checkpoints.
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
- Make sure that design minimizes constructs. Minimize current content.
    - [ ] Use nom or regex to create a syntax.
    - [ ] Design should be such that clumsy graph should be difficult to create.
    - [ ] Graph level; Measure of clumsiness and cleaning the graph. Detect clumsy additions on create. Suggest cleaning graph.
        - [ ] Untangling/Unwebbing.
        - [ ] Other topological sorts. Ex. Least scope sort. Least distance b/w connected nodes sort.
        - [ ] Remove unnecessary redundancies. For example, in `Let P(A) denote @probability@ of some @event@ A of a @stochastic experiment@ E` probability already is defined for an event and experiment. Should not have to declare it again.
        - [ ] Solve problem: For every new statement, I have to traverse entire graph and check for duplicates, optimize graph ...
    - [ ] Statement level; Measure of bigness of a statement. If too big indicate and split into multiple.
        - [ ] More terse text.
        - [ ] Limit/Soft limit on number of words in a description/significance/proof.
        - [ ] Vis the badness of the statement using this.
        - [ ] Typed/Syntaxed description.
    - [ ] A statement can have multiple references to a parent. That is not taken into account.
    - [ ] A statement can have a parent but not referenced. That is not taken into account.
    - [ ] Automatic theorem proving.
