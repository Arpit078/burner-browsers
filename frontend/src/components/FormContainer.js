import React, { useState } from 'react';
import './FormContainer.css';

function FormContainer() {
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [showPassword, setShowPassword] = useState(false);

  const handleSubmit = async (event) => {
    event.preventDefault();
    setLoading(true);

    try {
      const response = await fetch('http://127.0.0.1:3000/', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ password }),
      });

      if (!response.ok) {
        throw new Error('Failed to submit data');
      }

      const data = await response.json();
      const url = data.url;
      console.log(url)
      window.location.href = url;
    } catch (error) {
      console.error('Error submitting data:', error);
      alert('Failed to submit data. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const togglePasswordVisibility = () => {
    setShowPassword((prev) => !prev);
  };

  return (
      <div className="center">
        <form onSubmit={handleSubmit} className="form">
          <label htmlFor="password" className="label">
            <h2 className="subText">
              Create a temporary password for accessing the browser; you'll be asked for it when you login to the session.
            </h2>
          </label>
          <div className="passwordWrapper">
            <input
              type={showPassword ? 'text' : 'password'}
              id="password"
              name="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="input"
              required
            />
            <button
              type="button"
              onClick={togglePasswordVisibility}
              className="toggleButton"
            >
              {showPassword ? 'Hide' : 'Show'}
            </button>
          </div>
          <button type="submit" className="submitButton" disabled={loading}>
            {loading ? 'Starting...' : 'Start Browser'}
          </button>
        </form>
      </div>
  );
}

export default FormContainer;
