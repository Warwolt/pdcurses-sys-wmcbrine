This is a fork of the [pdcurses-sys](https://github.com/ihalila/pdcurses-sys)
repository, but changed to use the upstream win32
[wmcbrine/PDCurses](https://github.com/wmcbrine/PDCurses) source code for the
FFI bindings.

The fork was done because of an issue I had with working with the original
forked PDCurses backend, and success using the upstream wmcbrine backend.

I only needed the win32 flavor, so `wincon` is hardcoded as the version of
PDCurses that are used in these bindings.
