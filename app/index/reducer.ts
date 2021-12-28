export enum ActionType {
  SetAnimationID = "set-animation-id",
  SetSurface = "set-surface",
  Err = "err"
}

export interface Action {
  kind: ActionType;
  payload?: any;
}

export interface State {
  animationID: number | null;
  parametricSurface: Record<string, any> | null;
  error: boolean;
}

export const InitialState: State = {
  animationID: null,
  parametricSurface: null,
  error: false
}

export function reducer(state: State, action: Action): State {
  switch (action.kind) {
    case ActionType.Err:
      console.error(action.payload);
      return { ...state, error: true };
    case ActionType.SetAnimationID:
      console.log(`Currently running animation ID: ${action.payload}.`);
      return { ...state, animationID: action.payload };
    case ActionType.SetSurface:
      console.log("Successfully set parametric surface");
      return { ...state, parametricSurface: action.payload };
    default:
      console.log("Default action on Surface reducer.")
      return state;
  }
}
