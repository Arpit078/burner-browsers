import React from 'react';
import { useGoogleLogin } from '@react-oauth/google';

function SignIn() {
  const googleLogin = useGoogleLogin({
    onSuccess: (codeResponse) => {
      console.log(codeResponse.code)
      // Send the authorization code to the backend server
      fetch('http://localhost:5000/auth/jwt', {
        method: 'POST',
        headers: {
          // 'Authorization' : 'Bearer ' + codeResponse.access_token,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ code: codeResponse.code }),
      })
      .then(response => response.json())
      .then(data => {
        console.log('Backend response:', data);
      })
      .catch(error => {
        console.error('Error:', error);
      });
    },
    onError: () => {
      // Handle login errors here
      console.error('Google login failed');
    },
    flow: 'auth-code',
  });

  return (
    <button onClick={() => googleLogin()}>
      Sign in with Google
    </button>
  );
}

export default SignIn;