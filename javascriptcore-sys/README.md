# javascriptcore-sys

[![CI](https://github.com/endoli/javascriptcore.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/endoli/javascriptcore.rs/actions/workflows/ci.yml)
[![](https://img.shields.io/crates/v/javascriptcore-sys.svg)](https://crates.io/crates/javascriptcore-sys)
[![docs.rs](https://img.shields.io/docsrs/javascriptcore-sys)](https://docs.rs/javascriptcore-sys)

These are raw bindings for the JavaScriptCore library.

This only works on macOS and Linux for now, but should be
possible to use elsewhere once linking to the correct
libraries has been configured.

On Linux, you will need to have the correct packages installed.
On Debian and Ubuntu, this includes:

* `pkg-config`
* `libglib2.0-dev`
* `libjavascriptcoregtk-3.0-dev`, `libjavascriptcoregtk-4.0-dev`,
  or `libjavascriptcoregtk-4.1-dev`

Other Linux distributions may need different packages and changes
may need to be made to the ``build.rs`` script. Please submit bugs
or patches to help out with these sorts of portability concerns.

## Support and Maintenance

I am developing this library largely on my own so far. I am able
to offer support and maintenance, but would very much appreciate
donations via [Patreon](https://patreon.com/endoli). I can also
provide commercial support, so feel free to
[contact me](mailto:bruce.mitchener@gmail.com).

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
