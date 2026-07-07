# MAY Rust

MAY, for Make Agents Yourself, is a project from IRIT focused on defining and generating agent-oriented systems. The original project is available at <https://redmine.irit.fr/projects/may>.

This repository is a Rust redesign of MAY. The goal is to keep the language, tooling, and generated code easier to maintain while improving the reliability of code production. The current work focuses on parsing MAY/SPEADL source files into a structured AST, with formal proof artifacts added to document and check key grammar and AST invariants.

## Generation

Run the generator from `may_rust`:

```sh
cargo run -- -i ../examples/speadl/Simple.speadl -o ../examples/python --keep-intermediate
```

`--keep-intermediate` preserves backend-specific intermediate code. For Python generation, this writes the Python AST unparser script next to the generated `.py` file as `ComponentName.python_ast_unparse.py`.

Inputs and outputs can be provided with `-i`/`--input` and `-o`/`--output`. Both flags are repeatable. If no input is specified, the generator reads every `.speadl` file in `../examples/speadl` in alphabetical order. Input directories are also expanded alphabetically. If one output is provided for multiple inputs, it is treated as an output directory; repeated outputs are matched to the resulting input order.
