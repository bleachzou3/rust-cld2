[![Build Status](https://travis-ci.org/emk/rust-cld2.svg?branch=master)](https://travis-ci.org/emk/rust-cld2)

This Rust library detects the language of a string using the
[cld2 library][cld2] from the Chromium project.

To use it, add the following lines to your `Cargo.toml` file and run `cargo
update`:

```
[dependencies.cld2]
git = "git://github.com/emk/rust-cld2"
```

Then you can invoke it as follows:

``` rust
// Put these two lines the top of the file.
extern crate cld2;
use cld2::{detect_language, Format, Reliable, Lang};

let text = "It is an ancient Mariner,
And he stoppeth one of three.
'By thy long grey beard and glittering eye,
Now wherefore stopp'st thou me?";

assert_eq!((Some(Lang("en")), Reliable),
           detect_language(text, Format::Text));
```

You can also pass in language detection hints and request more detailed
output.  For details, please see [the API documentation][apidoc].

### Contributing

As always, pull requests are welcome!  Please keep any patches as simple as
possible and include unit tests; that makes it much easier for me to merge
them.

In your first commit message, please include the following statement:

> I dedicate any and all copyright interest in my contributions to this
project to the public domain. I make this dedication for the benefit of the
public at large and to the detriment of my heirs and successors. I intend
this dedication to be an overt act of relinquishment in perpetuity of all
present and future rights to this software under copyright law.

This allows us to keep the library legally unencumbered, and free for
everyone to use.

### License

The original cld2 library is distributed under the Apache License Version
2.0.  This also covers much of the code in `cld2-sys/src/wrapper.h`.  All
of the new code is released into the public domain as described by the
Unlicense.

[cld2]: https://code.google.com/p/cld2/
[apidoc]: http://www.rust-ci.org/emk/rust-cld2/doc/cld2/
