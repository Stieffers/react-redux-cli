// ------------------------------------
// Constants
// ------------------------------------
export const ${action_var_name} = "${action_name}";
export const ${action_var_name}_FAILED = "${action_name}_FAILED";

// ------------------------------------
// Actions
// ------------------------------------
export const handleSubmit = (event) => {
  event.preventDefault();
  return (dispatch, getState) => {
    fetch("", {
        body: {},
        method: "POST"
    }).then((response) => {
      dispatch({
        type: ${action_var_name},
        token: response.data
      });
    }).catch((error) => {
      dispatch({
        type: ${action_var_name}_FAILED,
        error: error.response.data.error
      })
    });
  }
};

export const actions = {
  handleFormSubmit,
};
