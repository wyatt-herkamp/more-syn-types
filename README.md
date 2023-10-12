# std_macros
A collection of macros from the rust standard library. Allows you to parse them in your own code

# Goals
- Get As Close as possible to the Rust Standard Library when parsing the macros
- Execute Macros such as `include_str!` and `include_bytes!` if you need to read it.


# Use Cases
- Need to read over a doc attribute to put into the macro generated code

### Contributing
If you notice something is not parsing that is allowed in the Rust Standard Library, please create a PR.

Feel free to parse nightly macros as well.

# Crate Features
- 'quote' - Enables adding parsed macro into quote
- 'executing' - Enables executing macros such as `include_str!` and `include_bytes!`
- 'extra-traits' - Add Debug to parsed macros