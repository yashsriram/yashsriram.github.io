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

### hard earned wisdom
- The best task management system until now is a white board and a marker.
- The best information note taking system is taking no notes at all.
- The best running notes are the ones that are forgotten shortly later.
- The best information gathering system until now is this.
- Learning by example and by answering questions is good.

### vision for this
- Make sure that design minimizes constructs. Minimize current content.
    - [ ] Design should be such that clumsy graph should be difficult to create.
    - [ ] Statement level; Measure of bigness of a statement. If too big indicate and split into multiple.
        - [ ] More terse text.
        - [ ] Limit/Soft limit on number of words in a description/significance/proof.
        - [ ] Vis the badness of the statement using this.
        - [ ] Typed/Syntaxed description.
    - [ ] Use `struct Definition<T>(phantomdata<T>)`, `struct SomeOperator<S1, S2>(S1, S2)`, `let set = Defn<And<Collection, Unique>>` to create strongly typed definitions to replace statment names and descriptions.
        - Arbitrary structs can be made representing axiom defns, operators, compund defn, syntax sugars defns, named defns. So anything that is written in stringy version of statemtents should be captured this way.
        - DAG structure is still preserved, although parent degree is now fixed at compile time i.e. you cannot easily have arbitrary number of parents which is probably is a good thing.
        - Debug and Display trait impl to recursively generate text versions of statements.
        - Basically makes string version of statements to typed defns (implement theorems later).
        - Still have to design operators and manage single copies of defns to create more defns. Any type can be a T of Defn<T>, which is too free.
        - [x] --Use nom or regex to create a syntax.--
    - [ ] Graph level; Measure of clumsiness and cleaning the graph. Detect clumsy additions on create. Suggest cleaning graph.
        - [ ] Untangling/Unwebbing.
        - [ ] Other topological sorts. Ex. Least scope sort. Least distance b/w connected nodes sort.
        - [ ] Remove unnecessary redundancies. For example, in `Let P(A) denote @probability@ of some @event@ A of a @stochastic experiment@ E` probability already is defined for an event and experiment. Should not have to declare it again.
        - [ ] Solve problem: For every new statement, I have to traverse entire graph and check for duplicates, optimize graph ...
    - [ ] A statement can have multiple references to a parent. That is not taken into account.
    - [ ] A statement can have a parent but not referenced. That is not taken into account.
    - [ ] Automatic theorem proving.
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
- Build a Server Client for browsing. Create & Read.
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
    - [x] --Left right to move across statements.-- Instead of that prev and next links at relatively same positions.
    - [x] Statement id list view/Statement complete view.
    - [x] Able to search in entire graph.
    - [x] Followable links to parents and children.
    - [x] Making checkpoints.
- [x] Graph representation. Graph page. DAG node mass caliberation. DAG node size caliberation. DAG statement heatmap. Btn to reset. Click to go to statement.
    - [ ] DAG search.
    - [ ] DAG latex type set description.
    - [ ] More control on rendering and ui (need force directed layout implementation i.e. spring-mass simulation).
