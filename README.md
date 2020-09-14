## json

A json parser, written in Rust using the `nom` parser combinator framework. This was a learning exercise for nom and writing parses. This was heavily inspired by the json implementaion linked from the nom site.

There seems to be a bug with  parsing `\`. The function `parse_str` is doesnt seem to parse a single `\`.

