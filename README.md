# eyeint (`ii`)

eyeint (`ii`) is a command line app to inspect arbitrary sized integers without various settings.

## Features

`eyeint` has the following features:

- Inspects integers with an arbitrary number of bits (up to 64)
- Converts arbitrary radix (2-36) integers into most common integer radices
  - binary
  - octal
  - decimal
  - hexadecimal
- Handles signed integers
- Truncates and (sign/zero) extends integers with arbitrary bit precision
- Shows information about an integer's two's complement

## Installation

You need to build `eyeint` from source. No precompiled binaries are available at the moment.

To build `eyeint`, you need a 2021 edition [Rust](rust) toolchain and [`cargo`](#cargo). Installing the Rust toolchain will likely install `cargo` for you. Once you have `cargo` installed, run the following:

```sh
# enter to the eyeint directory
$ cd eyeint
# compile and add the `ii` binary to cargo's bin
cargo install --path .
```

## Usage

The `$ ii -h` command yields the following help text:

```txt
eyeint 0.1.0
Sujal Bolia <sujalbolia@gmail.com>

eyeint (ii) inspects arbitrary length integers with various settings.

USAGE:
    ii [OPTIONS] <INPUT>

ARGS:
    <INPUT>    The integer to inspect

OPTIONS:
    -s, --signed             Treat the input as a signed integer
    -B, --byte               Treat the input as an 8-bit integer
    -S, --short              Treat the input as a 16-bit integer
    -i, --int                Treat the input as a 32-bit integer
    -l, --long               Treat the input as a 64-bit integer
        --bytes <BYTES>      Treat the input as integer of specified bytes
    -b, --bits <BITS>        Treat the input as integer of specified bits
    -e, --sign-extend        Sign-extend the input integer when converting it to a bigger size
        --zero-extend        Zero-extend the input integer when converting it to a bigger size
    -n, --binary             Treat the input as a binary (base-2) integer
    -o, --octal              Treat the input as an octal (base-8) integer
    -d, --decimal            Treat the input as a decimal (base-10) integer
    -x, --hexadecimal        Treat the input as a hexadecimal (base-16) integer
    -r, --radix <RADIX>      Treat the input as a integer of the specified radix
    -t, --twos-complement    Show information about the two's complement of the input as well
    -h, --help               Print help information
    -V, --version            Print version information
```

The complete help text can be viewed by running `$ ii --help`.

### Examples

Some example uses of `eyeint` are detailed below. Note that these examples do not exhibit every feature of `eyeint`.

```sh
# truncate to 4 bits
$ ii 0b101001 -b 4
Decimal         =>  9
Binary          =>  0b1001
Octal           =>  0o11
Hexadecimal     =>  0x9

Bits: 4

# truncate to 4 bits
# and treat it as a signed number
$ ii 0b101001 -b 4 -s
Decimal         =>  -7
Binary          =>  0b1001
Octal           =>  0o11
Hexadecimal     =>  0x9

Bits: 4

# zero-extend to a 12-bit
# integer to 16 bits.
$ ii 0x123 -S
Decimal         =>  291
Binary          =>  0b0000000100100011
Octal           =>  0o000443
Hexadecimal     =>  0x0123

Bits: 16

# sign-extend the input by treating it
# as a signed integer
$ ii 0x123 -S -s
Decimal         =>  -3805
Binary          =>  0b1111000100100011
Octal           =>  0o170443
Hexadecimal     =>  0xf123

Bits: 16
```

## Goals

- [ ] Handle integers with more than 64 bits
- [ ] Allow users to store their own default configuration
- [ ] Handle radices over 36
- [ ] Add control over the output

## Notes

Some reference notes:

- By default:
  - the input is treated as a base-10 integer unless a radix option/flag or a known prefix (`0b` - binary; `0[o|O]` - octal, `0x` - hexadecimal),
  - the bit size is equal to the minimum number of bits required to represent the input, and
  - the integer is zero-extended if it is unsigned and sign-extended if it is signed.
- Adding a negative sign before a number in a base other than decimal will likely lead to an unexpected result.
- I did not use the `bigint` crate as I made this project to primarily learn more about integers.

## License

`eyeint` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-MIT][mit] and [LICENSE-APACHE][apache] files for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[rust]: https://www.rust-lang.org/tools/install
[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
