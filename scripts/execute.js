const fs = require("fs");
const path = require("path");
const WABTModule = require("wabt");

const wasmInstanceOptions = {
  imports: {}
};

async function main() {
  const wabt = await WABTModule();
  const filepath = process.argv[2];
  const fileContents = fs.readFileSync(filepath, "utf-8");
  const moduleName = path.basename(filepath);
  const moduleBinary = wabt
    .parseWat(moduleName, fileContents)
    .toBinary({ log: false, write_debug_names: true })
    .buffer;
  const module = new WebAssembly.Module(moduleBinary);
  const instance = new WebAssembly.Instance(module, wasmInstanceOptions);
  const result = await instance.exports.main();
  console.log("Program returns:", result);
}

main();
