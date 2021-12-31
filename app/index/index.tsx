import * as React from "react";
import { useEffect, useReducer, useRef } from "react";
import Canvas from "@components/canvas";
import Selector from "@components/selector";
import { ActionType, reducer, InitialState } from "./reducer"
import "./index.css"

interface Props {
  wasmModule: Record<string, any>;
}

const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;
const FPS_THROTTLE = 1000.0 / 60;

const SHAPES = ["Torus", "Triforce"];

export default ({ wasmModule }: Props) => {
  const [state, dispatch] = useReducer(reducer, InitialState);
  const initMount = useRef(true);
  const initAnimation = useRef(true);
  const canvasRef = useRef(null);
  const initTime = Date.now();

  const animate = () => {
    const animationID = setInterval(() => {
      window.requestAnimationFrame(() => {
        const elapsedTime = (Date.now() - initTime) / 1000;
        state.parametricSurface.render(
          canvasRef.current.width,
          canvasRef.current.height,
          elapsedTime
        );
      })
    }, FPS_THROTTLE);

    dispatch({ kind: ActionType.SetAnimationID, payload: animationID });
  };

  // TODO: Much moist; make dry. 
  const swapSurface = (surface: string) => {
    const actionType = { kind: ActionType.SetSurface };

    switch (surface) {
      case "Torus":
        dispatch({ ...actionType, payload: new wasmModule.Torus("parametric-surface") });
        return;
      case "Triforce":
        dispatch({ ...actionType, payload: new wasmModule.Triforce("parametric-surface") });
        return;
      default:
        return
    }
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
      const triforce = new wasmModule.Triforce("parametric-surface");
      dispatch({ kind: ActionType.SetSurface, payload: triforce });
    } catch(e) {
      dispatch({ kind: ActionType.Err, payload: `Failed to initialize parametric surface with error: ${e}` });
      return;
    }
  });

  useEffect(() => {
    if (initAnimation.current) {
      initAnimation.current = false;
      animate();
      return;
    }

    console.log("Swapping parametric surface.")
    clearInterval(state.animationID);
    animate()
  }, [state.parametricSurface]);

  return (
    <div className="box">
      { state.error ? <h1 style={{ color: "white" }}>{"Something went wrong."}</h1> :

      <>
        <Selector
          options={SHAPES}
          callback={(s) => swapSurface(s)}
          style={{ marginTop: "75%" }}
        />
        <Canvas
          width={CANVAS_WIDTH}
          height={CANVAS_HEIGHT}
          ref={canvasRef}
        />
      </>

      }
    </div>
  )
}

