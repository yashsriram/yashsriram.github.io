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
