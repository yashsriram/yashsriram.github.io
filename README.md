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
- [x] Create a standalone html file which visualizes all statements in a directed acyclic graph.
- [x] Every statement has a family tree using which can be traversed using inline links to its parent statements.
- [x] Case combination check.
- [x] use only axioms and theorems, remove definitions.
- [x] DAG visualization.
- [x] DAG node mass caliberation.
- [x] DAG node size caliberation.
- [x] DAG statement heatmap.
- [ ] Simplify goal, design. Improve name. Make sure that design minimizes constructs.
- [ ] Typed system.
- [ ] TODOs.
- [ ] Use amethyst or similar to have more control on rendering and ui (need force directed layout implementation i.e. spring-mass simulation).
- [ ] DAG search.
- [ ] DAG btn to reset.
- [ ] DAG latex type set description.
- [ ] DAG click to pdf.
- [ ] more natural parent reference detection.
