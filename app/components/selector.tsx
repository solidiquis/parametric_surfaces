import * as React from "react";
import { useState } from "react";
import "./selector.css";

interface Props {
  options: Array<string>;
  callback: (message: string) => void;
  [x: string]: any;
}

export default ({ options, callback, ...props }: Props) => {
  const [selected, setSelected] = useState(options[0]);
  const [hover, setHover] = useState(false);
  const [expand, setExpand] = useState(false);

  const expandIcon = () => {
    return hover ? "\u25BE" : "\u25BF";
  };

  return (
    <div
      className="container"
      tabIndex={-1}
      onClick={() => setExpand(prev => !prev)}
      onBlur={() => setExpand(false)}
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
    >
      <div className="selection-box" {...props}>
        <div className="selected">{selected}</div>
        <div className={`expand-direction ${expand ? "fold-direction" : ""}`}>{expandIcon()}</div>
      </div>
      <div className={`options ${expand ? "" : "hide"}`}>
        {options.map((i) => (
          <div
            key={i}
            className="option"
            onClick={(e) => {
              const target = e.target as HTMLElement;
              setSelected(target.innerText);
              callback(target.innerText);
          }}>
            {i}
          </div>
        ))}
      </div>
    </div>
  );
}
