import { AuthClient } from "@dfinity/auth-client";
import  { useEffect, useState } from "react";
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import { Actor, HttpAgent } from "@dfinity/agent";
import { canisterId, idlFactory } from "./declarations/royaltiesCalculator";
import Navbar from "./Components/Navbar";
import Home from "./Components/Home";
import MessegeInput from "./Components/MessegeInput"

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
          ? `http://localhost:4943?canisterId=${"bd3sg-teaaa-aaaaa-qaaba-cai"}`
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

  let agent = new HttpAgent({
    host: env === "local" ? localhost : livehost,
    identity: identity,
  });

  //Only use agent.fetchRootKey() in development mode, in production remove this line
  agent.fetchRootKey();

  const backendActor = Actor.createActor(idlFactory, {
    agent,
    canisterId: canisterId,
  });

  return (
    <Router>
      <div className="bg-gray-900 text-white min-h-screen">
        <Navbar {...{ login, logout, isAuthenticated }} />
        <div>
          <Routes>
            <Route path="/" element={<Home {...{ isAuthenticated, backendActor }} />} />
            <Route path="add-message" element={<MessegeInput backendActor={backendActor} />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;
