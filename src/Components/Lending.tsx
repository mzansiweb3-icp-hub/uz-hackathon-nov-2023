import React, { useState, useEffect } from 'react';
import { useNavigate } from "react-router-dom";

function Lending({ backendActor, principal, isAuthenticated }) {
  const [deposit, setDeposit] = useState(0);
  const [isSending, setIsSending] = useState(false);
  const [isDepositSuccess, setDepositSuccess] = useState(false);

  const navigate = useNavigate();

  const makeDeposit = async (e) => {
    e.preventDefault();
    try {
      setIsSending(true);
      await backendActor.lendckEthToken(principal, deposit);
      setDeposit(0);
      setDepositSuccess(true);
      setIsSending(false);
      navigate('/dashboard');
    } catch (error) {
      console.log(error);
      setIsSending(false);
      navigate('/error-page');
    }
  };
  const returnHome = ()=> navigate('/');

  if (!isAuthenticated) {
    // If user is not authenticated, redirect to home page or handle accordingly
    returnHome();
  }

  return (
    <div className="flex justify-center items-center h-screen bg-gray-900 text-white">
      <div className="mx-auto max-w-lg p-6 bg-gray-800 rounded-md shadow-md">
        <h2 className="text-2xl font-bold mb-4">Deposit for Lending</h2>
        {!isDepositSuccess ? (
          <form onSubmit={makeDeposit} className="mb-4">
            <label className="block mb-2">Deposit Amount:</label>
            <input
              type="number"
              value={deposit}
              onChange={(e) => setDeposit(Number(e.target.value))}
              className="border border-gray-300 rounded-md px-2 py-1 mb-2 bg-gray-700 text-white"
            />
            <button
              type="submit"
              className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded block mx-auto"
            >
              {!isSending?"💰Deposit":"💸Depositing..."}
            </button>
          </form>
        ) : (
          <div className="mb-4">
            <p>Deposit Successful!</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Lending;
