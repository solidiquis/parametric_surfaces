import * as React from "react";
import { useEffect, useReducer, useRef } from "react";
import { ActionType, InitialState, reducer } from "./reducer";

// 60 frames per second
const FPS_THROTTLE: number = 1000 / 60;

interface Props {
  WasmModule: Record<string, any>
}

export default ({ WasmModule }: Props) => {
  const [state, dispatch] = useReducer(reducer, InitialState);
  const initMount = useRef(true);
  const initMount2 = useRef(true);
  const canvasRef = useRef(null);
  const initTime = Date.now();
  let lastDrawTime = -1; 

  const animate = () => {
    window.requestAnimationFrame(animate);
    
    const currTime = Date.now();

    if (currTime - lastDrawTime < FPS_THROTTLE)
      return;

    lastDrawTime = currTime;

    const elapsedTime = (currTime - initTime) / 1000;

    state.parametricSurface.render(canvasRef.current.width, canvasRef.current.height, elapsedTime);
  };

  useEffect(() => {
    if (!initMount.current) {
      return;
    }
    initMount.current = false;

    const gl = canvasRef.current.getContext("webgl", { antialias: true });

    if (!gl) {
      dispatch({ kind: ActionType.Err, payload: "Failed to intialize WebGL context" });
      return;
    } else {
      console.log("Successfully established WebGL context.");
    }

    try {
      const torus = new WasmModule.Torus("parametric-surface");
      dispatch({ kind: ActionType.SetSurface, payload: torus });
    } catch(e) {
      dispatch({ kind: ActionType.Err, payload: `Failed to initialize parametric surface with error: ${e}` });
      return;
    }
  });

  useEffect(() => {
    if (initMount2.current) {
      initMount2.current = false;
      return;
    }

    animate();
  }, [state.parametricSurface]);


  return (
    <div>
      {
        state.error ?
        <h1 style={{ color: "white" }}>{"Something went wrong..."}</h1> :
        <canvas
          id="parametric-surface"
          ref={canvasRef}
          height="600" width="800"
        ></canvas>
      }
    </div>
  )
}


//function Surface() {
  //const [state, dispatch] = useReducer(reducer, INITIAL_STATE);
  //const initMount = useRef(true);
  //const initMount2 = useRef(true);
  //const initMount3 = useRef(true);
  //const canvasRef = useRef(null);

  //var lastDrawTime = -1; 
  //var initTime = Date.now();

  //const animate = () => {
    //window.requestAnimationFrame(animate);
    
    //let currTime = Date.now();

    //if (currTime - lastDrawTime < FPS_THROTTLE)
      //return;

    //lastDrawTime = currTime;

    //let elapsedTime = (currTime - initTime) / 1000;

    //state.parametricSurface.render(canvasRef.current.width, canvasRef.current.height, elapsedTime);
  //};

  //useEffect(async () => {
    //if (!initMount.current) {
      //return;
    //}
    //initMount.current = false;

    //try {
      //let wasm = await WASM;
      //dispatch({ type: ActionType.SET_WASM, payload: wasm });
      //console.log("Successfully instantiated WASM module.")
    //} catch(e) {
      //dispatch({ type: ActionType.ERROR, payload: `Failed to instantiate WASM module with error: ${e}` });
      //return;
    //}

    //const gl = canvasRef.current.getContext("webgl", { antialias: true });

    //if (!gl) {
      //dispatch({ type: ActionType.ERROR, payload: "Failed to intialize WebGL context" });
      //return;
    //} else {
      //console.log("Successfully established WebGL context.");
    //}

  //});

  //useEffect(() => {
    //if (initMount2.current) {
      //initMount2.current = false;
      //return;
    //}

    //try {
      //let torus = new state.wasm.Torus("parametric-surface");
      //dispatch({ type: ActionType.SET_PARAMETRIC_SURFACE, payload: torus });
    //} catch(e) {
      //dispatch({ type: ActionType.ERROR, payload: `Failed to initialize parametric surface with error: ${e}` });
      //return;
    //}
  //}, [state.wasm]);

  //useEffect(() => {
    //if (initMount3.current) {
      //initMount3.current = false;
      //return;
    //}

    //animate()
  //}, [state.parametricSurface]);

  //return (
    //<>
      //{
        //state.error ?
        //<h1 style={{ color: "white" }}>{"Something went wrong..."}</h1> :
        //<canvas
          //id="parametric-surface"
          //ref={canvasRef}
          //height="600" width="800"
        //></canvas>
      //}
    //</>
  //)
//}


