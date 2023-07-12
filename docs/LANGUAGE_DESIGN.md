# BearLang

BearLang is a toy language to practice a few things:
- End-to-End compiler implementations
- WASM

BearLang will compile down to WASM.

# Features

## Typing

BearLang is a statically typed language.

## Types

BearLang has the following types:
- Number (Float64)
- Array (Array of Float64)
- Void (Zero-sized type)

## Variable

Variables can be defined with the keyword `var`:

Example:

```
var x = 100.0;
var y = 12.0;
y = 37.0;
var z = [1.0, 3.4];
```

Note that the type of variables will be inferred.


Variables are scoped, i.e. the following code will fail to compile:

```
{
    var x = 100;
}
print(x)
```

## Control flow - If

The syntax for `if` statements is:

```
if (x == 100) {
    print(100)
} else if (x == 200) {
    print(200)
} else {
    print(300)
};
```

## Control flow - While loops

The syntax for `while` loops is:

```
while (x > 0) {
    print(x);
    x = x - 1;
};
```

## Functions

Functions are defined using the `function` keyword as follows:

```
function getExampleFirstField(example: Number): Number {
    return index(example, 0);
}
```

Note that the return value is required, even if it can be inferred. If the return value is not set it will be assumed to be Void. To return a value from a function the `return` keyword should be used:

```
function getExampleField(example: Array, fieldNum: Number): Number {
    return index(example, fieldNum) * 2;
}
```

Two function can share the same name as long as they do not share the same type signature.

## Operators

Are functions on two variables that instead of being written in a prefix format are written in an infix format. The operators available on BearLang are:

- `+`
- `-`
- `*`
- `/`
- `^`
- `==`
- `=`
- `>`
- `>=`
- `<`
- `<=`

To overload an operator one can write:

```
operator +(lhs: Number, rhs: Number): Number {
    lhs + rhs
}
```

## Entrypoint

The entrypoint for the program is a function named `main` which takes an Array as input.

## Code splitting

BearLang will feature no code-splitting functionality.

## Built-in functions

The following built-in functions are available:

- `print`: outputs to STDOUT.
- `index`: get the element at a particular index of an array.
- `length`: returns size of an array.

## Memory management

TODO
