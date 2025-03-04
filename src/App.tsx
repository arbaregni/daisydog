import { useState } from 'react'
import dogImage from './assets/dog.jpg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <h1>Wanted: Public Menace</h1>
      <div>
          <center>
              <img src={dogImage} alt="Daisy the Dog" />
          </center>
      </div>
      <h1>Have you seen this dog???</h1>
      <center>
          <div className="card">
            <button type="button" onClick={() => setCount((count) => count + 1)}>
              count is {count}
            </button>
          </div>
      </center>
    </>
  )
}

export default App
