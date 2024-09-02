import './App.css';
import { GoogleOAuthProvider } from '@react-oauth/google';
import SignIn from './components/SignIn';
import GoogleLoginButton from './components/GoogleLoginButton';
function App() {
  return (
    <GoogleLoginButton/>
  );
}

export default App;
