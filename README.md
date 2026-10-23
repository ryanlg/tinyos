# tinyos

A small, Unix-like hobby kernel written in Rust, built from scratch with as many corners cut as reasonably possible.

## Why
> _Yeah, I know, it's yet another hobby kernel._

This project was inspired by [a Reddit post](https://www.reddit.com/r/osdev/s/8K6E99eD6m) whose author claimed to have written a Unix-like kernel from scratch in 12 hours, without letting LLMs write the code. I was certainly skeptical, like most of the commenters.

I don't particularly care whether the claim is true. I want to find out for myself how far a deliberately minimal kernel can get when speed takes priority over completeness. This is not a speedrun or a competition. There is no time limit and no single sitting - I just keep a tally of the hours spent.

**End goal:** run `echo "hello world" > output.txt` in BusyBox.

## Ground Rules

- **AI as an oracle is allowed:** It's just searching Google, only faster.
- **AI as a code generator is not allowed:** Avoiding that is the whole point.
- **Copy-pasting is allowed, with attribution:** Existing code usually needs heavy changes anyway. But I can't copy *a lot*, or I might as well fork someone else's kernel and call it a day. Borrowed code is marked with `@attrib:` comments.
- **No external crates:** Everything is built on `core` alone, otherwise I could just pull in `x86_64`, `spin`, or the rest of the [blog_os](https://os.phil-opp.com/) code.
- **Corners will be cut:** See below.
- **The code should still be clean and presentable:** Otherwise I'd lose interest quickly.
- **Rust:** It's fun (~~and to make bare-metal code harder to write~~).

## Corners Cut

Features a modern OS would have that are deliberately left out or simplified:

- Booted by GRUB (Multiboot 1), no custom bootloader
- 32-bit (i686) only
- No SMP: a single core
- Identity-mapped address space
- In-memory, linked-list-based filesystem
- No networking

This list will grow as I find more corners to cut.

## Building and Running

Dependencies come from the Nix flake (`nix develop`, or `direnv allow`), which provides nightly Rust, `nasm`, `lld`, QEMU and `xorriso`. You also need `i686-elf-grub-mkrescue` on your `PATH` outside of Nix because that trips Nix's system triple check.

```sh
make            # debug build, and make kernel ISO
make qemu       # build and boot in QEMU
```
