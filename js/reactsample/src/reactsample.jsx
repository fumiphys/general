import React from 'react';
import ReactDOM from 'react-dom';

class App extends React.Component {
  render(){
    return (
      <div className="reactdiv">React App(renew)</div>
    )
  }
}

class Instancereact extends React.Component {
  constructor(props){
    super(props)
    this.state = {innertext: 'empty'}
  }

  onChange(e){
    this.setState({innertext: e.target.value})
  }

  render(){
    return (
      <div>
        <input type="text" onChange={this.onChange.bind(this)} />
        <p>{this.state.innertext}</p>
      </div>
    )
  }
}

//ReactDOM.render(
//  <App />,
//  document.getElementById("samplediv")
//)

ReactDOM.render(
  <Instancereact />,
  document.getElementById("samplediv")
)
