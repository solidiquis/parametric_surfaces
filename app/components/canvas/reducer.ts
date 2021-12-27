export enum ActionType {
  SetWasm = "set-wasm",
  SetSurface = "set-surface",
  Err = "err"
}

export interface Action {
  kind: ActionType;
  payload?: any;
}

export interface State {
  wasm: Record<string, any>;
  error: boolean;
  parametricSurface: Record<string, any>;
}

export const InitialState: State = {
  wasm: null,
  error: false,
  parametricSurface: null
}

export function reducer(state: State, action: Action): State {
  switch (action.kind) {
    case ActionType.SetWasm:
      return { ...state, wasm: action.payload };
    case ActionType.Err:
      console.error(action.payload);
      return { ...state, error: true };
    case ActionType.SetSurface:
      console.log("Successfully set parametric surface");
      return { ...state, parametricSurface: action.payload };
    default:
      console.log("Default action on Surface reducer.")
      return state;
  }
}
