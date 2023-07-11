const fs = require("fs");
const path = require("path");
const WABTModule = require("wabt");

function print(num) { console.log(num); }

const wasmInstanceOptions = {
  imports: {
    print,
  }
};

async function main() {
  const wabt = await WABTModule();
  const filepath = "./samples/basic-wasm/basic-wasm.wat";
  const fileContents = fs.readFileSync(filepath, "utf-8");
  const moduleName = path.basename(filepath);
  const moduleBinary = wabt
    .parseWat(moduleName, fileContents)
    .toBinary({ log: false, write_debug_names: true })
    .buffer;
  const module = new WebAssembly.Module(moduleBinary);
  const _ = new WebAssembly.Instance(module, wasmInstanceOptions);
}

main();
