import  { useEffect, useState } from "react";
import { Route, Routes, BrowserRouter } from 'react-router-dom'
import { Actor, HttpAgent } from "@dfinity/agent";
import { canisterId, idlFactory } from "./declarations/datasource";
import Home from "./components/home"
import NavBar from "./components/navBar"
import CreateLanguage from "./components/createLanguage"
import AddTopics from "./components/addTopics"
import ViewStories from "./components/viewStories"
import TellStories from "./components/tellStories"

const env = process.env.DFX_NETWORK || "local";
const localhost = "http://localhost:4943";
const livehost = "https://icp0.io";

const App = () => {
  let agent = new HttpAgent({
    host: env === "local" ? localhost : livehost,
  });

  //Only use agent.fetchRootKey() in development mode, in production remove this line
  agent.fetchRootKey();

  const backendActor = Actor.createActor(idlFactory, {
    agent,
    canisterId: canisterId,
  });
  return(
    <BrowserRouter>
      <NavBar />
      <Routes>
        <Route path="/" element = {<Home {...{backendActor}} />} />
        <Route path="/createLanguage" element = {<CreateLanguage backendActor={backendActor} />} />
        <Route path="/addTopics" element = {<AddTopics backendActor={backendActor} />} />
        <Route path="/viewStories" element = {<ViewStories backendActor={backendActor} />} />
        <Route path="/tellStories" element = {<TellStories backendActor = {backendActor} />} />
        </Routes>
    </BrowserRouter>
  )
};

export default App;
