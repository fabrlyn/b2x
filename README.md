# b2x

`b2x` is a cli tool and library for converting between data formats and types.

Common use-cases involve converting binary to hexadecimal or hexadecimal to utf-8.

## Usage

### Binary

#### From binary to decimal
```
b2x bin dec 1011 1010

=> 11 10
```

###### Flags

- `--float`

- `--signed`

- `--big-endian`

- `--group-size [2-64]`

#### From binary to hexadecimal
```
b2x bin hex 11010 11011

=> 0x1A 0x1B
```

#### From binary to ASCII
```
b2x bin ascii 01001111 01001110

=> ON
```

#### From binary to UTF-8
```
b2x bin utf-8 11000010 10110101

=> µ
```

