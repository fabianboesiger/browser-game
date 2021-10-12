import React from 'react';
import './Form.css';

// Listener is superclass for all components that
// need to listen to the websocket stream.
class Listener extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      response: "",
    }
    // TODO: Deriving the keyword to listen to form the class name is probably not ideal.
    this.keyword = this.constructor.name.toLowerCase();
  }

  componentDidMount() {
    this.props.ws.addEventListener("message", e => {
      const parsed = JSON.parse(e.data);
      if (parsed[this.keyword] !== undefined) {
        const response = parsed[this.keyword];
        console.log("received " + this.keyword + ": " + response);
        this.setState({ response: response });
      }
    })
  }
}

// Form handles sending and receiving form data.
class Form extends Listener {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }
  
  handleChange(e) {
    this.setState({ [e.target.name]: e.target.value });
  }

  handleSubmit(e) {
    e.preventDefault();
    let payload = {};
    payload[this.keyword] = this.state;
    this.props.ws.send(JSON.stringify(payload))
    console.log("sent " + this.keyword + ": " + payload);
  }
}

export default Form;
