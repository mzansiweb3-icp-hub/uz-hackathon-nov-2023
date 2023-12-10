import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

function Dashboard({ backendActor, principal, isAuthenticated }) {
  const [debt, setDebt] = useState(0n);
  const [collateral, setCollateral] = useState(0n);
  const [amtLend, setAmtLend] = useState(0n);

  useEffect(() => {
    const fetchData = async () => {
      const debtValue = await backendActor.getDebtInformation(principal);
      const collateralToken = await backendActor.getCollateralIcpToken(principal);
      const amountLanded = await backendActor.getAmountLanded(principal);
      setDebt(debtValue || 0n); // Default to 0 if value is undefined
      setCollateral(collateralToken || 0n);
      setAmtLend(amountLanded || 0n);
    };
    fetchData();
  }, [principal]);

  return (
    <div className="bg-gray-900 text-white min-h-screen flex flex-col justify-center items-center">
      <h1 className="text-4xl font-bold mb-6">Dashboard</h1>
      <div className="text-center mt-12">
        <p>Deposited: {collateral.toString()}</p>
        <p>Borrowed: {debt.toString()}</p>
        <p>Landed: {amtLend.toString()}</p>
      </div>
      <div className="mt-8">
        <Link to="/" className="text-blue-400 hover:text-blue-600">
          Go to Home
        </Link>
      </div>
    </div>
  );
}

export default Dashboard;
