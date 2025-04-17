<div align="center">

# `rust-cli-template`

**Rust template with cli, logging and pretty backtraces.**

</div>

## Features

* Logging:
    * Logging verbose to console in debug mode.
    * Logging non blocking and less verbose to console in release mode.
    * Logging non blocking as json to file in release mode.
* CLI:
    * Licenses of dependencies are automatically parsed at build and by default available with the `--licenses` flag.
* Pretty Backtraces:
    * Pretty backtraces and spantraces are supported through color eyre.
    * Backtraces are fully shown in debug mode and omitted in release mode.
    * A link to create a github issue is supplied in release mode.


## Warning

This template pulls in a fair bit of dependencies. This means that the compile is slow from the get go.
Though not slower than using color eyre and tracing in your own project.


## License

This template is under the [Unlicense license](https://unlicense.org/).
