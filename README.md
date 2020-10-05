# journal

## description
My perspective and understanding of mathematics.

## code
- All source code is in `main.py` `statement.py` `utils.py` and is written in `python3`.
- `templatex/` contains mako template files. The content is written in them.
- The structure of content is illustrated in the first few pages.
- Incomplete things are marked with `TODO` for tracking purposes.
- There is some content not part of dag in pdf.
- The templates are parsed, processed and a latex and a html outputs are generated.
    - All latex files are generated under `tex/`.
    - Html file generated is `graph.html`.

## documentation
- The documentation of the code is itself.

## usage
- Use `python3 main.py` to compile templatex files to latex files in `tex/` and to generate a html canvas graph in `graph.html`.
- Use `cd tex && latexmk -pdf main.tex` to compile latex files into a pdf.
- The pdf is designed for laptop/desktop screens in with dark mode enabled in pdf reader.
- Each statement is on a new page to improve reading experience.
- To search for a statement/TODOs use pdf reader search feature.

## roadmap
- [ ] Todos in text.
- [ ] Simplify goal, design. Make sure that design minimizes constructs.
- [ ] Improve name.
- Design
    - [x] Use only axioms and theorems, remove definitions.
    - [x] Use only statements, remove axioms and theorems.
    - [ ] More natural parent reference detection.
- Text representation.
    - [x] Case combination check.
    - [x] Every statement has a family tree. One can traverse using inline links to its parent statements.
- Minimize current content.
    - [ ] More terse text.
    - [ ] Remove unnecessary redundancies. For example, in `Let P(A) denote @probability@ of some @event@ A of a @stochastic experiment@ E` probability already is defined for an event and experiment. Should not have to declare it again.
- Static typing
    - [ ] Introduce syntax and semantics.
    - [ ] Syntax is normal rust.
    - [ ] First class TODOs, partials support.
    - [ ] Need some latex like type setting.
- Graphical representation.
    - [x] Create a standalone html file which visualizes all statements in a directed acyclic graph.
    - [x] DAG node mass caliberation.
    - [x] DAG node size caliberation.
    - [x] DAG statement heatmap.
    - [ ] DAG search, btn to reset, DAG latex type set description, DAG click to pdf.
    - [ ] More control on rendering and ui (need force directed layout implementation i.e. spring-mass simulation).
- Build a TUI for browsing.
    - [ ] CRUD of statements.
    - [ ] ORM of statements.
    - [ ] Statement id list view/Statement complete view, with chapter separators.
    - [ ] Able to search in entire graph.
    - [ ] Followable links to parents.
    - [ ] List children of node.
    - [ ] Colors.
    - [ ] Compile and open the statement PDF. Should be snappy.
    - [ ] Handle TODOs and scratch statements.
    - [ ] Open graphical view with keymap.
