osom_encoders_x86_64
====================

This crate provides encoders for some common X86_64 instructions and entities.

Build
-----

Typically you would only run

> cargo build

as usual. However if any change happens to `x86.yaml` file, you will need to
regenerate automatically generated files. Do that by switching to
`_osom_encoders_x86_64_generator` project and run

> cargo run -- --destination ../osom_encoders_x86_64/src
