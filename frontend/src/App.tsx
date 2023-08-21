import { useEffect, useState } from 'react'
import './App.css'
import axios from 'axios';

type Penguin = {
  id: string,
  penguin_name: string,
  age: number,
};


function App() {
  const [penguins, setPenguins] = useState<Penguin[]>([]);

  useEffect(() => {
   fetchPenguins();
  }, []);
  
  const fetchPenguins = async () => {
    const response = await axios.get(`http://localhost:8000/api/penguins`);
    setPenguins(response.data);
  }

  return (
    <>
      {penguins.map(penguin => <div key={penguin.id}>{penguin.penguin_name}</div>)}
    </>
  )
}

export default App
