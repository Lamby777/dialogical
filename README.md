# dialogical

Dialogue editor toolkit tailor-made for P/E/T/S 2037's dialogue system. This is
the same tool I'll be using for actual vanilla dialogue, so modders will be
getting basically first-class support by using this... unless the
open-sourcerors decide to make their own tool, which is certified epic.

WIP.

## WTF is this?

Pretty much a "compiler" for RPG character dialogue. You can write dialogue in a
bunch of text files in a format that's easy for git to track/diff, and this tool
will do all the work of converting it to a table of dialogue nodes that your
game can load into memory.

## Building

Just `cargo build`. If you're here from P/E/T/S, you don't actually need to
install this system-wide anymore, as it's now a build dependency and does all
the work using the library instead of running system commands. The only reasons
you'd need to install this separately are if you want to use it as a standalone
tool or if you're helping to develop it.

## Usage

All of these commands will print the help screen. Pick your favorite, and don't
forget it! :>

```
dg
dg -h
dg --help
```

Here's an example of how you'd use it from the command line...

```
dg pets37.dg -o ./pets37.dgc
```

For a full project, you'd ideally slap this line into your build script or
Makefile or something like that before all your other build steps.

Here's how it works in P/E/T/S, as an example.

```rs
fn main() {
    println!("cargo:rerun-if-changed=./dg/src");
    println!("cargo:rerun-if-changed=build.rs");

    dialogical::compile("dg/src/main.dg", "dg/packed.dgc").unwrap();
}
```

---

As per usual, xkcd has a relevant comic describing this project:
![xkcd 927: Standards](https://imgs.xkcd.com/comics/standards.png)

Jokes aside, I don't think this project would be too useful for people making
their own games. It's pretty much tailor-made for P/E/T/S 2037 and I won't be
adding support for stuff other people need unless it's something I also happen
to need for P/E/T/S. If you want to use it for your own game or make a fork for
your own needs, feel free to do so, but I'm really just making this open-source
for educational purposes and to make modding easier.
