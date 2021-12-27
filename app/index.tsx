import "core-js/stable";
import "regenerator-runtime/runtime";
import * as React from "react";
import * as ReactDOM from "react-dom";

import Canvas from "@components/canvas/canvas";

import("../pkg/index").then(wasm => {
  ReactDOM.render(
    <Canvas WasmModule={wasm} />,
    document.getElementById("root")
  );
}).catch(e => {
  alert("Failed to instantiate WASM module.")
  console.error(e);
})
