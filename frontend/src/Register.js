import React from 'react';
import Form from './Form';

class Register extends Form {
  constructor(props) {
    super(props);
    this.state = {
      username: "",
      password: ""
    }
    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  render() {
    return (
      <form onSubmit={this.handleSubmit}>
        <h2>Register</h2>
        <label htmlFor="username">Username</label>
        <input id="username" name="username" type="text" value={this.state.username} onChange={this.handleChange}></input>
        <label htmlFor="password">Password</label>
        <input id="password" name="password" type="password" value={this.state.password} onChange={this.handleChange}></input>
        <input type="submit"></input>
        <p>{this.state.response}</p>
      </form>
    )
  }
}

export default Register;
