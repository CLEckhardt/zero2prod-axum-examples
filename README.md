# zero2prod `axum` examples

In case you are working through the book
[Zero To Production In Rust](https://www.zero2prod.com/) but want to use `axum`
instead of `actix-web`, this repo contains examples to help with that
translation.

This repo will __not__ contain all `actix-web` code in the book translated into
`axum`. It will __only__ contain the minimum examples required to help you
implement the patterns in Zero2Prod in `axum`.

## Why is this necessary?

While `actix-web` and `axum` serve the same purpose, they were developed via
different approaches. Because of this, the Zero2Prod pattern translations from
`actix-web` to `axum` aren't one-to-one; we can't simply swap `actix_web::` for
`axum::`. This repo demonstrates the changes we must make to use `axum` with
the Zero2Prod patterns.

