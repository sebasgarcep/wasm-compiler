# BearLang

BearLang is a toy language to practice a few things:
- End-to-End compiler implementations
- WASM

BearLang will compile down to WASM.

# Features

## Typing

BearLang is a statically typed language.

## Primite types

BearLang has the following primitive types:
- Boolean (`true` and `false` keywords)
- Int64 (Numbers without a dot)
- Float64 (Numbers with a dot)
- String (Denoted by `"..."`, stored as a reference to a char array to ensure size)
- Void (Empty, zero-sized type)

## Special types

- None: Special struct with zero size.
- Array: Each sized type, with non-zero size, also has an associated array type. For example Boolean's associated array type is Boolean[]. Can create arrays with the notation `[valueA, valueB]`.
- Optional: Each sized type, with non-zero size, also has an associated optional type. For example Boolean's associated array type is Boolean?. This type is equivalent to an enum with variants None and Boolean. Can create optionals with the notation `Boolean?(true)` or `Boolean?(None())` for the null value.

## Arrays

Arrays are a special type that is generated for every built-in and user-generated type.

## Variable

Variables can be defined with the keyword `var`:

Example:

```
var x: Int64 = 100;
var y: Float64 = 12.0;
y = 37.0;
```

Note that the type of variables can be inferred, making the following code equivalent to the previous one:

```
var x = 100;
var y = 12.0;
y = 37.0;
```

Variables are scoped, i.e. the following code will fail to compile:

```
{
    var x: Int64 = 100;
}
print(x)
```

## Statements as expressions

All statements in BearLang are expressions.

## Scopes as expressions

A scope is itself an expression and the final line is the return value. Therefore the following code is perfectly valid code on BearLang:

```
var y = {
    var x = 100;
    x * 2;
};
```

## Structs

A struct is defined with the keyword `struct`. For example:

```
struct Example(
    i: Int64,
    j: Float64,
);
```

and structs can be initialized using the struct name:

```
var x = Example(300, 1.5);
```

Any valid sized type can become a struct field, and a struct can have no fields (useful for enums which we will discuss in the next section).

## Enums

Enums can be declared using the `enum` keyword:

```
struct FirstVariant(x: Int64);

struct SecondVariant(y: Float64);

enum ExampleEnum(FirstVariant, SecondVariant);
```

We can initialize an enum with:

```
var enumValue = ExampleEnum(FirstVariant(100));
```

## Control flow - If

The syntax for `if` statements is:

```
if (x == 100) {
    print("A")
} else if (x == 200) {
    print("B")
} else {
    print("C")
};
```

Each arm of the `if` statement can return a value:

```
var y = 
    if (x == 100) {
        150
    } else if (x == 200) {
        250
    } else {
        x
    };
```

## Control flow - While loops

The syntax for `while` loops is:

```
while (x > 0) {
    print(String(x));
    x = x - 1;
};
```

## Control flow - Match statements

The syntax for `match` statements is:

```
match (enumValue) (
    FirstVariant(x) {
        print(to_string(x))
    },
    else {
        print("Not Found")
    },
);
```

If we want to return a value from a `match` statement we can:

```
var y = match (enumValue) (
    FirstVariant(x) {
        x
    },
    else {
        0
    },
);
```

## Functions

Functions are defined using the `function` keyword as follows:

```
function getExampleFirstField(example: Example): Int64 {
    example.i
}
```

Note that the return value is required, even if it can be inferred. If the return value is not set it will be assumed to be Void. If you wish to early-return or to explicitly return at the end of a function, the `return` keyword can be used:

```
function getExampleField(example: Example, fieldNum: Int64): Int64 {
    if (fieldNum == 1) {
        return example.i;
    }
    example.i * 2
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
- `&`
- `|`
- `>>`
- `<<`
- `and`
- `or`
- `xor`

To overload an operator one can write:

```
operator +(lhs: Example, rhs: Example): Int64 {
    lhs.i + rhs.i
}
```

## Entrypoint

The entrypoint for the program is a function named `main` which takes an array of string values as input.

## Code splitting

BearLang will feature no code-splitting functionality.

## Built-in functions

The following built-in functions are available:

- `print`: outputs a string to STDOUT.
- `not`: negates a boolean value.
- `to_string`: converts a value into a string.
- `index`: get the element at a particular index of an array. Returns Optional.
- `length`: returns size of an array.

## Memory management

TODO
