// import * as wasm from "hello-wasm-pack";

// wasm.greet();

// For more comments about what's going on here, check out the `hello_world`
// example.
import("../pkg/wasm_canvas")
  .catch(console.error);