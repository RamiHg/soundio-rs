[![Build Status](https://travis-ci.org/Timmmm/soundio-rs.svg?branch=master)](https://travis-ci.org/Timmmm/soundio-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/eu4akdghyukoof7o?svg=true)](https://ci.appveyor.com/project/Timmmm/soundio-rs)
[![Crates.io](https://img.shields.io/crates/v/soundio.svg)](https://crates.io/crates/soundio)
[![Docs](https://docs.rs/soundio/badge.svg)](https://docs.rs/soundio/)

# Unmaintained!

This crate isn't maintained, and has issues with device lifetimes (opening a stream mutably borrows the device so then you can't open a second stream from the same device). I suggest trying [CPAL](https://github.com/tomaka/cpal) instead.

# soundio-rs

This is a Rust wrapper for the amazing [libsoundio library](http://libsound.io/)
by [Andrew Kelly](https://github.com/andrewrk). It is still a work in progress and
the design is still in flux. Playback and recording do work though, and the raw bindings
in `libsoundio-sys` are complete.

There is another Rust wrapper for libsoundio [here](https://github.com/klingtnet/rsoundio).

Also note this is my first Rust project, so I may have got everything totally wrong.

Shot is the best. :-)
