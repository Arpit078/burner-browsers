import React from 'react'
import "./Home.css"
import FormContainer from '../components/FormContainer.js'

function Home() {
  return (
    <main className="wrapper">
          <div className="title">
            <p>Burner Browsers</p>
          </div>
    
          <div className="sub_text">
            Docker browsers in the cloud for you to feel safe in the shady jungle of Internet :)
          </div>      
          <FormContainer/>    
    </main>
  )
}

export default Home