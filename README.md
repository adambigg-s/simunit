================================================================================
simunit
================================================================================

Lightweight Rust library for dimensional analysis and unit checking. The goal
is to tack on some metadata to numeric types holding their physical units. This
allows automatic detection of operations that are dimensionally invalid.

Heavily inspired by MATLAB's `symunit`

CORE CONCEPT
------------

There are two primary concepts:
  `Unit` Rust type representing a physical dimension
  `Value<V>` a numeric value with dimensionality tracked

This isn't for symbolic mathematics, any operations will be eagerly evaluated.
The important thing is the dimensional information is kept together

RELEASE BUILDS
--------------

MATLAB's `symunit` adds something like 10,000x runtime overhead, as all the
math is done symbolically. This doesn't do that, so the overhead is relatively
minimal.

When running with "--release", the dimensional tracking is completely removed,
and `Value<V>` should collapse to the underlying type. So, this library incurs
absolutely zero runtime overhead in a release build.
