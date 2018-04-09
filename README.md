# passgenr

**THIS CODE IS UNDER CONSTRUCTION, IT SHOULD NOT BE RELIED ON TO GENERATE SECURE
PASSWORDS YET.**

[![Build Status](https://travis-ci.org/defuse/passgenr.svg?branch=master)](https://travis-ci.org/defuse/passgenr)

`passgenr` is a Rust library for generating cryptographically-secure random
passwords. It is a port of my earlier password generating tool,
[passgen](https://github.com/defuse/passgen) (which was written in C).
`passgenr` also includes a command-line utility for generating passwords.

## Build Instructions

TODO

## Command-Line Utility Examples

```
$ passgenr --ascii
|*H(f]*@XO;YX"vEOx_%3LDf}fyAuQ<_2=&W<|d*ZY#zH%{Wq20mruTo:G~jg-rd

$ passgenr --alpha
PWPMBzF4KIUNGIK79S04NOgt51s5TJaqCTNd4loMkTjIZiHsrGMUrqE4DCrBCuay

$ passgenr --hex
0FB8DA7DF897D3E781D8F93D48A1FDA19C4B1CA96A3D78E1CB1BE46441AD7EE3

$ passgenr --digit
4685009459776989842380332148352094362440679705765781117806140754

$ passgenr --lower
bjqxtuknhlqacsiwjansyavkaqlnyscsnxwowcgymlkwxzlilxbzsyovyoqwjdmw

$ passgenr --words
vocalist.uptown.bunch.feel.board.crock.few.teeter.product.intellect

$ passgen --hex -p 5
753924DC422047A0D9FFDDEE87BCF6BA65D992EE317178D1C77BDE46DAC13C42
1ABFFDA08CD24BBD34590D183EE25C610A6B9CCD9847081A786B0061EF312769
2C065D5BD06412C6BE08C47F728D8AB9A099B5C42102517897426D9CF5420DCA
239EDCE8E3788F8E86383411EBA7A3E819F8897C263327AA20503D563E59733B
C2A980F8DFCC686F389B5CB96D30701C22D0B7B6BF2D732F7CD1364D81D949CC
```

## Library Examples

This crate is on [crates.io TODO actual link](TODO). To use it, add it to your
dependencies in `Cargo.toml`...

```
[dependencies]
passgenr = "0.1"
```

...and add this to your crate root (`lib.rs` if you're writing a library, or
`main.rs` if you're writing a binary):

```
extern crate passgenr;
```

TODO example

TODO link to full docs


## Security Details

**Randomness.** `passgenr` uses [OsRng](https://doc.rust-lang.org/rand/rand/struct.OsRng.html)
as a randomness source, which reads directly from the operating system's CSPRNG
(e.g. `getrandom(2)` or `/dev/urandom` on Linux, or `RtlGenRandom` on Windows).
The individual elements of the password (characters or words) are selected by
calling `.choose()` on the `OsRng`, which is careful to sample uniformly (i.e.
it doesn't use the naive "mod N" algorithm).

**Swap File.** `passgenr` does not prevent its memory from being written out to
the system's swap file. You should only use `passgenr` on systems with an
encrypted swap file/partition.

**Side-Channels.** Unlike the older `passgen`, `passgenr` has no defenses
against side-channel attacks. When you generate passwords with `passgenr` you
should be sure that nobody untrustworthy can run code on the same machine (even
as an unprivileged user), otherwise they might be able to extract some
information about the passwords through side-channels. Adding side-channel
defenses to `passgenr` is
[planned](https://github.com/defuse/passgenr/issues/4), once the technology
becomes available to stable Rust.

**Audit Status.** While its author is careful and prioritizes security,
`passgenr` has not yet been audited by a professional third-party. If you have
experience doing security audits and you would like to contribute one, please
get in touch!

