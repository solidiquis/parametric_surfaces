export enum ActionType {
  SetSurface = "set-surface",
  Err = "err"
}

export interface Action {
  kind: ActionType;
  payload?: any;
}

export interface State {
  parametricSurface: Record<string, any> | null;
  error: boolean;
}

export const InitialState: State = {
  parametricSurface: null,
  error: false
}

export function reducer(state: State, action: Action): State {
  switch (action.kind) {
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
