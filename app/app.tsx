import "core-js/stable";
import "regenerator-runtime/runtime";
import * as React from "react";
import * as ReactDOM from "react-dom";
import Index from "@index/index";

import("../pkg/index").then(wasm => {
  ReactDOM.render(
    <Index wasmModule={wasm} />,
    document.getElementById("root")
  );
}).catch(e => {
  alert("Failed to instantiate WASM module.")
  console.error(e);
})
