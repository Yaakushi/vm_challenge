# Synacor Challenge

The original repository this was forked from holds a copy of the arch-spec and
the program binary for the Synacor challenge.

Please refrain from reading if you want to solve it on your own, as there might
be "spoilers" below. Or at least brief considerations of tools used to solve
this that might be considered spoilers by some people.

This repo holds my attempt at a solution written in Rust. At the time of writing
this, I'm currently attempting to learn Rust. I apologize if anyone ever stumbles
across this code, it's probably not very rust idiomatic, and perhaps just outright
bad in a language agnostic way.

## Coin permutator

At one point in the "game", we're asked to solve an equation with blank values 
using some coins. Each coin represents a value, so we're supposed to find a 
permutation that solves it.

The bin crate `coin_permutator` solves that.

It can be invoked using `cargo run --bin=coin_permutator`.

## Codes

Codes below are the md5sum hashes of the codes found through the challenge.

Codes with strikethrough are codes I've personally found already.

To take the md5sum in a *nix PC:

```console
$ echo -n "<Code Here>" | md5sum
6fcd818224b42f563e10b91b4f2a5ae8  -
```

- ~76ec2408e8fe3f1753c25db51efd8eb3~
- ~0e6aa7be1f68d930926d72b3741a145c~
- ~7997a3b2941eab92c1c0345d5747b420~
- ~186f842951c0dcfe8838af1e7222b7d4~
- ~2bf84e54b95ce97aefd9fc920451fc45~
- ~e09640936b3ef532b7b8e83ce8f125f4~
- 4873cf6b76f62ac7d5a53605b2535a0c
- d0c54d4ed7f943280ce3e19532dbb1a6
