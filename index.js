import react from "react";
import ReactDOM from "react-dom";
const rust = import("./pkg/index");

function Test() {
  return (
    <>
      <h1>Hello World</h1>
    </>
  )
}

const canvas = document.getElementById("parametric-surface");
const gl = canvas.getContext('webgl', { antialias: true });

window.onload = rust.then(wasm => {
  if (!gl) {
    alert("Failed to initialize WebGL");
    return;
  }

  let torus;
  try {
    torus = new wasm.Torus("parametric-surface");
  } catch(e) {
    alert("Failed to initialize parametric surface.");
    console.error(e);
    return;
  }

  const FPS_THROTTLE = 1000 / 60; // 60fps
  var lastDrawTime = -1; 
  var initTime = Date.now();

  function render() {
    window.requestAnimationFrame(render);
    
    let currTime = Date.now();

    if (currTime - lastDrawTime < FPS_THROTTLE)
      return;

    lastDrawTime = currTime;

    let elapsedTime = (currTime - initTime) / 1000;

    torus.render(canvas.width, canvas.height, elapsedTime);
  }

  render();
});
