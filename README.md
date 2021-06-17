# oxyserve

This is the source code for the webserver of my website, [andi-makes.dev](https://andi-makes.dev).

## Used Technology
This webserver is written in `rust` using the `rocket` crate. I am using `handlebars` as the templating engine.

`rocket` requires `rust nightly` as of the time of writing. Because of incompabilities with `rust-analyzer`, a rust language server, I'm using `nightly-2021-05-20`. To enable this specific
`rust nightly` version for this project, run the following command:
```
rustup override set nightly-2021-05-20
```
