import { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import axios from 'axios';



function App() {

  const [notes, setNotes] = useState([]);

  useEffect(() => {
  
    axios.get('http://localhost:8080/notes')
    .then(response => {
      console.log(response.data);
      setNotes(response.data);
    })
    .catch(error => {
      console.log(error);
    });

  }, []);
  


  return (
    <div className="App">
      <header className="App-header">
        <h1>Your Notes</h1>
        <ul>
      {notes.map(note => (
        <li key={note.id}>
            <h1>{note.title}</h1>
            <p>{note.content}</p>
            <p>{note.importance}</p>
            <p>{note.done}</p>
            <h4>
              {new Date(note.date).toLocaleString("de-DE", {
              year: "numeric",
              month: "2-digit",
              day: "2-digit",
              hour: "2-digit",
              minute: "2-digit"
            })}
            </h4>
          </li>
      ))}
      </ul>
      </header>
    </div>
  );
}

export default App;
