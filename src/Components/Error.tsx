import React from 'react';

function ErrorPage() {
  return (
    <div className="flex justify-center items-center h-screen bg-gray-900 text-white">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-4">Oops! Something went wrong.</h1>
        <p className="text-lg">We apologize, but it seems an error occurred.</p>
        <p className="text-lg">Please try again later or contact support.</p>
      </div>
    </div>
  );
}

export default ErrorPage;
