import * as React from "react";
import { forwardRef } from "react";

interface Props {
  height: number;
  width: number;
}

type Ref = HTMLCanvasElement;

export default forwardRef<Ref, Props>(({ height, width }, ref) => {
  return (
    <div>
      <canvas
        id="parametric-surface" // id necessary for Rust-land
        ref={ref}
        height={height} width={width}
      ></canvas>
    </div>
  )
})
