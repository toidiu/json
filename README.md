## json

A json parser, written in Rust using the `nom` parser combinator framework.

There seems to be a bug with  parsing `\`. The function `parse_str` is doesnt seem to parse a single `\`.
