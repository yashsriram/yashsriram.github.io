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

- Make sure that design minimizes constructs. Minimize current content.
    ```html
    <h2 id="why-this-effort">why this effort?</h2>

    <p>Well, I contemplate about a lot of things and it seems that after a point I am not able to keep track of them. I forget some important conclusions, perspectives, schools of thought, jewels of wisdom and such things in various aspects that are fruits of a lot of effort, suffering and pain. And I often have to re-start my train of thought on them, which is tedious and demotivating after a point of time.</p>

    <h5 id="therefore-this-journal-is-to">Therefore this journal is to</h5>

    <ol>
    <li><p>Write down stuff</p>
    <ol>
    <li><p>In a simplified, clean, elegant, minimalistic form.</p></li>
    <li><p>So that I don’t lose it permanently.</p></li>
    <li><p>So that I don’t have to remember what I don’t need to.</p></li>
    <li><p>So that I use my brain for thinking rather than storing.</p></li>
    </ol></li>
    <li><p>Use the power of machine search.</p></li>
    <li><p>Reuse previous knowledge. Don’t start over from scratch unnecessarily.</p></li>
    </ol>

    <img class="pure-img" src="/static/why.png"/>

    <h5 id="the-effort-will-be-to">The effort will be to</h5>

    <ol>
    <li><p>Build constructs that are simple, minimal, atomic, composable and excel at one thing.</p></li>
    <li><p>Build meaningful connections and flow among constructs.</p></li>
    <li><p>Keep everything as simple and minimal as possible.</p></li>
    <li><p>Promote recognition rather than re-call.</p></li>
    </ol>

    <p>These ideas are inspired by the UNIX philosophy, my experience in programming and doing things in general.</p>

    <h2 id="the-process-of-human-learning">the process of human learning.</h2>

    <p>The word ‘human’ is important, as mathematics, science, philosophy etc... are after all, human made things and there is nothing ‘absolute’ or ‘exact’ about them. So, there is no reason to be serious or stuck up with anything (especially science). Beliefs lies at the heart of all human made things (even science and the mighty math). As beliefs are not unconditionally correct, nothing should be blindly accepted forever, rather everything better be constantly challenged. This does not mean that nothing should be trusted, that might be even worse.</p>

    <p>It is in the <strong>balance</strong> between having trust and challenging it simultaneously lies any <strong>conceivable learning</strong>. If the balance tips left it is called ‘arrogance’, else if it tips right it is called ‘insanity’. If the balance is right it produces a sense of happiness, fun and joy.</p>

    <p>I believe that this feeling is what I am after. Science, math, whatever are just classifications of work but the balance is the real deal. <strong>In fact, discovering joy through this balance is the spirit of life itself</strong>.</p>

    <img class="pure-img" src="/static/learning.png"/>
    ```
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
