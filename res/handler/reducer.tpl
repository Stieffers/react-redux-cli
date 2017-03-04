// ------------------------------------
// Action Handlers
// ------------------------------------
const ACTION_HANDLERS = {
  [${action_var_name}]: (state, action) => {
    return Object.assign({}, state, action, {error: null});
  },
  [${action_var_name}_FAILED]: (state, action) => {
    return Object.assign({}, state, {error: action.error});
  }
};

// ------------------------------------
// Reducer
// ------------------------------------
const initialState = {error: null};
export default function ${lower_name}Reducer(state = initialState, action) {
  const handler = ACTION_HANDLERS[action.type];

  return handler ? handler(state, action) : state
}