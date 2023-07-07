const fs = require("fs");

const wasmModule = new WebAssembly.Module(fs.readFileSync("./samples/basic.wasm"));

function print(num) { console.log(num); }

const wasmInstance =
      new WebAssembly.Instance(wasmModule, {
        imports: {
          print,
        }
      });
