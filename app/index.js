import "core-js/stable";
import "regenerator-runtime/runtime";
import React, { useEffect, useRef, useReducer } from "react";
import ReactDOM from "react-dom";

const WASM = import("rust/index");
const FPS_THROTTLE = 1000 / 60;

const INITIAL_STATE = {
  wasm: null,
  error: false,
  parametricSurface: null
};

const ACTIONS = {
  SET_WASM: "set-wasm",
  ERROR: "error",
  SET_PARAMETRIC_SURFACE: "set-parametric-surface"
};

function reducer(state, action) {
  switch (action.type) {
    case ACTIONS.SET_WASM:
      return { ...state, wasm: action.payload };
    case ACTIONS.ERROR:
      console.error(action.payload);
      return { ...state, error: true };
    case ACTIONS.SET_PARAMETRIC_SURFACE:
      console.log("Successfully set parametric surface");
      return { ...state, parametricSurface: action.payload };
    default:
      console.log("Default action on Surface reducer.")
      return state;
  }
}

function Surface() {
  const [state, dispatch] = useReducer(reducer, INITIAL_STATE);
  const initMount = useRef(true);
  const initMount2 = useRef(true);
  const initMount3 = useRef(true);
  const canvasRef = useRef(null);

  var lastDrawTime = -1; 
  var initTime = Date.now();

  const animate = () => {
    window.requestAnimationFrame(animate);
    
    let currTime = Date.now();

    if (currTime - lastDrawTime < FPS_THROTTLE)
      return;

    lastDrawTime = currTime;

    let elapsedTime = (currTime - initTime) / 1000;

    state.parametricSurface.render(canvasRef.current.width, canvasRef.current.height, elapsedTime);
  };

  useEffect(async () => {
    if (!initMount.current) {
      return;
    }
    initMount.current = false;

    try {
      let wasm = await WASM;
      dispatch({ type: ACTIONS.SET_WASM, payload: wasm });
      console.log("Successfully instantiated WASM module.")
    } catch(e) {
      dispatch({ type: ACTIONS.ERROR, payload: `Failed to instantiate WASM module with error: ${e}` });
      return;
    }

    const gl = canvasRef.current.getContext("webgl", { antialias: true });

    if (!gl) {
      dispatch({ type: ACTIONS.ERROR, payload: "Failed to intialize WebGL context" });
      return;
    } else {
      console.log("Successfully established WebGL context.");
    }

  });

  useEffect(() => {
    if (initMount2.current) {
      initMount2.current = false;
      return;
    }

    try {
      let torus = new state.wasm.Torus("parametric-surface");
      dispatch({ type: ACTIONS.SET_PARAMETRIC_SURFACE, payload: torus });
    } catch(e) {
      dispatch({ type: ACTIONS.ERROR, payload: `Failed to initialize parametric surface with error: ${e}` });
      return;
    }
  }, [state.wasm]);

  useEffect(() => {
    if (initMount3.current) {
      initMount3.current = false;
      return;
    }

    animate()
  }, [state.parametricSurface]);

  return (
    <>
      {
        state.error ?
        <h1 style={{ color: "white" }}>{"Something went wrong..."}</h1> :
        <canvas
          id="parametric-surface"
          ref={canvasRef}
          height="600" width="800"
        ></canvas>
      }
    </>
  )
}

ReactDOM.render(<Surface />, document.getElementById("root"));

