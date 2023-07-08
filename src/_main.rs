enum WebAssemblyPrimitiveType {
    I32,
    I64,
    F32,
    F64,
}

enum WebAssemblyPrimitiveValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

enum WebAssemblyInstruction {
    Const {
        value: WebAssemblyPrimitiveValue
    },
    Call {

    },
}

enum WebAssemblyExpression {
    Module {
        expressions: Vec<WebAssemblyExpression>,
    },
    Function {
        name: String,
        arguments: Vec<>,
        output: Option<WebAssemblyPrimitiveType>,
        instructions: Option<Vec<WebAssemblyInstruction>>,
    },
    Variable {
        name: String,
    },
    Start {
        entrypoint: Box<WebAssemblyExpression>,
    },
}

fn main() {
    println!("Hello, world!");
}
