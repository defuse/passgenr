# passgenr

[![Build Status](https://travis-ci.org/defuse/passgenr.svg?branch=master)](https://travis-ci.org/defuse/passgenr)
[![crates.io](https://img.shields.io/crates/d/passgenr.svg)](https://crates.io/crates/passgenr)
[![crates.io](https://img.shields.io/crates/v/passgenr.svg)](https://crates.io/crates/passgenr)
[![docs.rs](https://docs.rs/passgenr/badge.svg)](https://docs.rs/passgenr/)

`passgenr` is a Rust library for generating cryptographically-secure random
passwords. It is a port of my earlier password generating tool,
[passgen](https://github.com/defuse/passgen) (which was written in C).
`passgenr` also includes a command-line utility for generating passwords. Please
read the [Security Details](#security-details) section below.

## Command-Line Utility

To build and install the command-line utility, run...

```
cargo build --bin passgenr --release
```

...and then install the `./target/release/passgenr` into your system.

Here are some examples of how to use the command-line tool:

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

$ passgenr --hex -p 5
0E21238E1B35FE6B38890AF83CBC1DD3470EE30F31971ECF49170CEE593D0312
1057CA652A62EA045B58EF2FA31077CA8749936D4FA87931EE22E4CC36BFBA02
2548942BB7A11D793225BD4E2B84E3FCBD66118F28C4C3871823745779340A30
878C14A4BD2C9F7B76C09D0A1A308AD471F4E06B13DC96886CAEAB2446E33178
1F0E1C337872EECE8FFC89A4088875CEB22BB5956B38D0C62FC28855202AB1F5
```

## Library

This library is [on crates.io](https://crates.io/crates/passgenr). The
documentation is hosted [on docs.rs](https://docs.rs/passgenr/).

To use the library, add the following to your `Cargo.toml`...

```
[dependencies]
passgenr = "0.2"
```

...and add this line to your crate root...

```
extern crate passgenr;
```

...now you can generate a password...

```
assert_eq!(
    20,
    passgenr::random_password(passgenr::charsets::ASCII, 20, "").unwrap().len()
);
```

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

