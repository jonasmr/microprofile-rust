[![Build Status](https://travis-ci.org/jonasmr/microprofile-rust.svg?branch=master)](https://travis-ci.org/jonasmr/microprofile-rust)

# microprofile-rust

microprofile-rust is a profiler for profiling and optimizing multithreaded game code

* github: https://github.com/jonasmr/microprofile-rust.git

Its a rust port of microprofile:

* github: https://github.com/jonasmr/microprofile.git

Which in turn depends on the excellent distorm project

* github: https://github.com/gdabah/distorm


# Integration
Add microprofile as a dependency:
```
[dependencies]
microprofile = "0.2.1"
```

microprofile is meant to be called from a game engine every frame.
to do this you call `microprofile::flip` every frame.
If your program is not framebased, you can use `microprofile::start_auto_flip(delay_in_ms)` and `microprofile::stop_auto_flip()`, which will make microprofile call this function itself every `delay_in_ms` ms.

To time some code, you insert a scope in your program
```
microprofile::scope!("group-name", "timer-name");
```

All of microprofile-rust's functions are available as macros. Use the feature "disabled" to fully disable all the macros, and remove all the microprofile code instrumentation

```
[dependencies.microprofile]
version = "0.2.1"
features = ["disabled"]
```

# Limitations
As of version 0.2.1 only a subset of microprofile is supported:
* Scope timers are supported
* Dynamic instrumentation is supported
* Windows, Linux and OSX is supported

The following features are _not_ supported:
* GPU timers
* Counters

# Using the UI
Please see the microprofile documentation : https://github.com/jonasmr/microprofile

# Example code
* https://github.com/jonasmr/microprofile-rust-demo

# Updates
* 0.2.0: Update to microprofile 3.1: Firefox support, quality of life features for programs with -many- threads
* 0.2.1: Update to latest microprofile: Fix a nasty perf regression in the live view.

