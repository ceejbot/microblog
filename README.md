# microblog Rust live-coding demo

This is a partially-implemented RESTful API server that is the backend for a microblogging service, which is a excitingly novel idea that might be *the* hot startup of 2005. It was written to be the subject of a live-coding in Rust exercise to show off the language to experienced programmers who do not already know Rust. The comments in the code are aimed at that audience.

There are deliberate errors in this code to demonstrate how `cargo check` and `cargo clippy` work, as well as unimplemented functions and missing derives.

Requirements: a Rust toolchain and a local running postgres. To perform your own live-coding high-wire routine, copy dot-env to `.env`, run `createdb microblog`, and then run `cargo check`. Have at it!

If you finish the route implementation during your demo, you can go on to generate OpenAPI docs for the service using [aide and the example here](https://docs.rs/aide/latest/aide/axum/index.html).

## LICENSE

If you insist. This code is licensed via [the Parity Public License.](https://paritylicense.com) This license requires people who build on top of this source code to share their work with the community, too.
