# journal

## description
Collection of my thoughts on various things in my life.

## code
- The content is written in mako templates for latex (files with templatex extension).
- The templates are read using python3.
- All source code is in `main.py` `statement.py` & `utils.py`.
- Things to do are marked with `TODO` for tracking purposes.
- The templates are parsed, processed and an output is generated.
- During processing, a directed acyclic graph (dag) is built on content.
- Output can be
    - A text version of the dag (topologically sorted nodes in compilable latex files), or
    - A graphical version of the dag (on a html canvas).
    - By default both are generated.
- The structure of content is illustrated in the first few pages.
- There is some content not part of dag in pdf.

## documentation
- The documentation of the code is itself.

## usage
- Use `python3 main.py` to compile templatex files to latex files in `tex/` and to a html canvas graph in `graph.html`.
- Use `cd tex && latexmk -pdf main.tex` to compile latex files into a pdf.

## roadmap
- [ ] Todos.
- Design
    - [x] Use only axioms and theorems, remove definitions.
    - [ ] More natural parent reference detection.
- Text representation.
    - [x] Case combination check.
    - [x] Every statement has a family tree. One can traverse using inline links to its parent statements.
- Graphical representation.
    - [x] Create a standalone html file which visualizes all statements in a directed acyclic graph.
    - [x] DAG node mass caliberation.
    - [x] DAG node size caliberation.
    - [x] DAG statement heatmap.
    - [ ] DAG search.
    - [ ] DAG btn to reset.
    - [ ] DAG latex type set description.
    - [ ] DAG click to pdf.
    - [ ] More control on rendering and ui (need force directed layout implementation i.e. spring-mass simulation).

- Minimize current content.
    - [ ] More terse text.
    - [ ] Remove unnecessary redundancies. For example, in `Let P(A) denote @probability@ of some @event@ A of a @stochastic experiment@ E` probability already is defined for an event and experiment. Should not have to declare it again.
- Build a TUI for browsing.
    - [ ] TUI.
    - [ ] Per statement view.
    - [ ] Per chapter view.
    - [ ] Followable links to parents.
    - [ ] Colors.
    - [ ] List children of node.
    - [ ] Open graphical view with keymap.
    - [ ] Able to search in entire graph.
- [ ] Introduce syntax and semantics.
- [ ] Syntax is normal rust.
- [ ] First class TODOs, partials support.
- [ ] Need some latex like type setting.
- [ ] Simplify goal, design. Make sure that design minimizes constructs.
- [ ] Improve name.
