import { AuthClient } from "@dfinity/auth-client";
import { useState, useEffect } from "react";
import { BrowserRouter as Router, Route, Routes, Link } from 'react-router-dom'
import { Actor, HttpAgent } from "@dfinity/agent";
import { canisterId, idlFactory } from "./declarations/micro_defi";
import { canisterId as interneId } from "./declarations/internet_identity";
import Navbar from "./Components/Navbar";
import Home from "./Components/Home";
import Borrow from "./Components/Borrow";
import Lending from "./Components/Lending";
import RepayLoan from "./Components/RepayLoan";
import ErrorPage from "./Components/Error";
import Dashboard from "./Components/Dashboard";

const env = process.env.DFX_NETWORK || "local";
const localhost = "http://localhost:4943";
const livehost = "https://icp0.io";

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const login = async () => {
    const authClient = await AuthClient.create({
      idleOptions: {
        idleTimeout: 1000 * 60 * 30,
        disableDefaultIdleCallback: true,
      },
    });
    await authClient.login({
      identityProvider:
        env === "local"
          ? `http://localhost:4943?canisterId=${interneId}`
          : "https://identity.ic0.app/#authorize",
      onSuccess: () => {
        checkAuth();
      },
    });
  };

  const checkAuth = async () => {
    try {
      const authClient = await AuthClient.create();
      if (await authClient.isAuthenticated()) {
        setIsAuthenticated(true);
      }
    } catch (error) {
      console.log("Error on check auth ", error);
    }
  };

  const logout = async () => {
    const authClient = await AuthClient.create();
    await authClient.logout();
    setIsAuthenticated(false);
  };

  useEffect(() => {
    checkAuth();
  }, []);

  const [identity, setIdentity] = useState<any>();

  useEffect(()=>{
    const initializeAuthClient = async () => {
      const client = await AuthClient.create();
      const id = client.getIdentity();
      setIdentity(id);
    };
    initializeAuthClient();
  }, [])
  const principal = identity?.getPrincipal().toString();
  console.log(`Principal Id: ${principal}`);

  let agent = new HttpAgent({
    host: env === "local" ? localhost : livehost,
    identity: identity,
  });

  //Only use agent.fetchRootKey() in development mode, in production remove this line
  // agent.fetchRootKey();

  const backendActor = Actor.createActor(idlFactory, {
    agent,
    canisterId: canisterId,
  });

  return (
    <Router>
      <div className=" bg-gray-900 text-white min-h-screen">
        <Navbar {...{isAuthenticated, login, logout}} />
        <div className="mx-auto">
          <Routes>
            <Route path="/" element={<Home {...{backendActor, principal, isAuthenticated}}/>} />
            <Route path="borrow" element={<Borrow {...{backendActor, principal, isAuthenticated}} />} />
            <Route path="lend" element={<Lending {...{backendActor, principal, isAuthenticated}} />} />
            <Route path='repay-loan' element={<RepayLoan {...{backendActor, principal, isAuthenticated}}/>}/>
            <Route path="dashboard" element={<Dashboard {...{backendActor, principal, isAuthenticated}}/>}/>
            <Route path='/*' element={<ErrorPage/>}/>
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;

