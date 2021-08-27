import React from 'react';
import './App.css';
import Register from './Register';
import Login from './Login';


function App() {
  return (
    <div>
      <Game></Game>
    </div>
  );
}

class Square extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      value: 0,
    }
  }

  render() {
    return (
      <button onClick={() => this.setState({value: this.state.value + 1})}>
        {this.state.value}
      </button>
    )
  }
}

class Board extends React.Component {
  render() {
    return (
      <div>
        <Square></Square>
      </div>
    )
  }
}

class Game extends React.Component {
  ws = new WebSocket("ws://localhost:8000");

  componentDidMount() {
    this.ws.onopen = () => {
      console.log("Connected to the websocket.");
    }

    this.ws.onclose = () => {
      console.log("Disconnected from the websocket.")
    }
  }

  render() {
    return (
      <div>
        <Register ws={this.ws}></Register>
        <Login ws={this.ws}></Login>
        <Board></Board>
      </div>  
    )
  }
}

export default App;
