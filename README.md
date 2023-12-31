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

Want to get this up and running so you can compile P/E/T/S? Simple as shit.

```
cargo install dialogical
```

If you want to actually work on this specific tool, or you need the latest
version for whatever reason, then just `cargo build` inside the folder.

Probably gonna write a crate for loading these files into memory at some
point... or maybe I'll just make that be a part of the game's code to avoid the
extra complexity of spamming a bunch of wrapper types just to add traits to
types from external crates.

## Usage

All of these commands will print the help screen. Pick your favorite, and don't
forget it!

```
dg
dg -h
dg --help
```

Here's an example of how you'd use it for a project... Ideally, you'd slap this
line into your build script or Makefile or something like that before all your
other build steps.

```
dg pets37.dg -o ./pets37.dgc
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
