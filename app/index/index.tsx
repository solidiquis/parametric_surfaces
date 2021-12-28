import * as React from "react";
import { useEffect, useReducer, useRef } from "react";
import Canvas from "@components/canvas";
import { ActionType, reducer, InitialState } from "./reducer"

interface Props {
  wasmModule: Record<string, any>;
}

const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;
const FPS_THROTTLE = 1000.0 / 60;

export default ({ wasmModule }: Props) => {
  const [state, dispatch] = useReducer(reducer, InitialState);
  const initMount = useRef(true);
  const initMount2 = useRef(true);
  const canvasRef = useRef(null);
  const initTime = Date.now();
  let lastDrawTime = -1;

  const animate = () => {
    const animationID = setInterval(() => {
      window.requestAnimationFrame(() => {
        const currTime = Date.now();

        if (currTime - lastDrawTime < FPS_THROTTLE)
          return;

        lastDrawTime = currTime;

        const elapsedTime = (currTime - initTime) / 1000;

        state.parametricSurface.render(canvasRef.current.width, canvasRef.current.height, elapsedTime);
      })
    }, FPS_THROTTLE);

    dispatch({ kind: ActionType.SetAnimationID, payload: animationID });
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
      const torus = new wasmModule.Torus("parametric-surface");
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
      { state.error ? <h1 style={{ color: "white" }}>{"Something went wrong."}</h1> :
        <Canvas
          width={CANVAS_WIDTH}
          height={CANVAS_HEIGHT}
          ref={canvasRef}
        />
      }
    </div>
  )
}

