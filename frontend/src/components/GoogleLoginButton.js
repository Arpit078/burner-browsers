// src/components/GoogleLoginButton.js
import { GoogleOAuthProvider, GoogleLogin } from '@react-oauth/google';
// import axios from 'axios';

const GoogleLoginButton = () => {
  const handleSuccess = async (response) => {
    const idToken = response.credential;
    try {
    //   const res = await axios.post('http://localhost:5000/auth/google', { idToken });
    console.log(idToken)
    //   console.log('Authentication successful:', res.data);
    } catch (error) {
      console.error('Error authenticating:', error);
    }
  };

  return (
    <GoogleOAuthProvider clientId="205747903130-g14qcjlk0chgfel32no1fq1e9kas9o15.apps.googleusercontent.com">
      <GoogleLogin onSuccess={handleSuccess} onError={() => console.log('Login failed')} />
    </GoogleOAuthProvider>
  );
};

export default GoogleLoginButton;
