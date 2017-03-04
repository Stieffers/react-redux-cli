import { connect } from 'react-redux'

import ${name}View from '../components/${name}View';
import {handleSubmit} from "../actions/${name}";

const mapDispatchToProps = {
  handleSubmit
};

const mapStateToProps = (state) => ({
  error: state.auth.error
});

export default connect(mapStateToProps, mapDispatchToProps)(${name}View)
