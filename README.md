# b2x

`b2x` is a cli tool and library for converting between data formats and types.

Common use-cases involve converting binary to hexadecimal or hexadecimal to utf-8.

## Usage

```sh
b2x <from> <to> <input>
```

### Binary

```sh
b2x bin <to> <input>
```

###### Parameters

##### `to`

The conversion target to convert the binary bit string to

- dec
- f32
- f64
- hex
- ascii
- utf-8

##### `input`

The binary bit string to convert

For example, `1010 1001` or just `10101001`

#### From binary to decimal integer

###### Command

```sh
b2x bin dec <input>
```

###### Flags

##### `--big-endian`

Interpret `input` binary string bits as big endinan

Little endian by default

##### `--group-size`

Set a fixed size of the input binary string bit groupings.
The default behaviour is to use space as the value delimiter

- optional
- default: uses spaces as value delimiter
- values: 2-128

##### `--signed`

Interpret `input` binary string as signed values.

Unsigned by default

##### `--spaced`

Indicate if binary numbers are seperated by a space character

- optional
- default: false

###### Example
```
b2x bin dec 1011 1010

=> 11 10
```

#### From binary to decimal float

###### Command

```sh
b2x bin f32 <input>
```

```sh
b2x bin f64 <input>
```

###### Flags

##### `--big-endian`

Interpret `input` binary string bits as big endinan

Little endian by default

###### Example

```
b2x bin f32 1011 1010

=> 11 10
```

```
b2x bin f64 1011 1010

=> 11 10
```

#### From binary to hexadecimal integer

###### Command
```
b2x bin hex <input>
```

###### Example

```
b2x bin hex 11010 11011

=> 0x1A 0x1B
```

#### From binary to ASCII

###### Command

```
b2x bin ascii <input>
```

###### Example

```
b2x bin ascii 01001111 01001110

=> ON
```

#### From binary to UTF-8

###### Command

```
b2x bin utf-8 <input>
```

###### Example

```
b2x bin utf-8 11000010 10110101

=> µ
```

