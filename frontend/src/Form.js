import React from 'react';
import './Form.css';

class Listener extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      response: "",
    }
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

class Form extends Listener {
  constructor(props) {
    super(props, "register");
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
