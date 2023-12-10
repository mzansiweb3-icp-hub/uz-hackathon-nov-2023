import { AuthClient } from "@dfinity/auth-client";
import { useState, useEffect } from "react";
import { Link } from 'react-router-dom'

const Home = ({ backendActor, principal, isAuthenticated }) => {
  const [hasDebt, setHasDebt] = useState(false);
  const [welcomePhrase] = useState("Welcome to Micro-Exchange - Where Crypto Meets Convenience!");

  const fetchData = async()=>{
    try {
      // Fetch the deposited amount from the backendActor
      const debt = await backendActor.getHasDebt(principal);
      console.log(debt);
      setHasDebt(debt[0]);
    } catch (error) {
      console.log(error);
    }
  }

  useEffect(() => {
    fetchData();
  }, [principal]);


  return (
    <div className="bg-blue-900 text-white min-h-screen flex flex-col justify-center items-center">
      <h1 className="text-4xl font-bold mb-6">{welcomePhrase}</h1>
      {isAuthenticated ? (
        <div className="text-center mt-12">
          <h2 className="text-4xl font-bold mb-6">Welcome back!</h2>
          <p className="text-lg">
            Start exploring and exchanging cryptocurrencies hassle-free!
          </p>
          <div className="flex justify-center mt-8 space-x-8">


            <Link to={!hasDebt ? "/borrow" : "/repay-loan"}>
              <button className="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded text-lg">
                {!hasDebt ? "💰Borrow" : "💸Repay"}
              </button>
            </Link>
            <Link to={hasDebt ? "/error-page" :"/lend"}>
              <button className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded text-lg">
                {!hasDebt ? "✅ Lend" : "❌ Lend"}
              </button>
            </Link>
          </div>
        </div>
      ) : (
        <div className="text-center mt-12">
          <h5 className="text-4xl font-bold mb-6">Please log in to start borrowing and lending ETH on ICP with ease!</h5>
        </div>
      )}
    </div>
  );
};

export default Home;
