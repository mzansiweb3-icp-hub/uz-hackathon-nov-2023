import { Link } from "react-router-dom"; // Import Link from React Router

const Navbar = ({ login, logout, isAuthenticated }) => {
  return (
    <div className="flex justify-between py-10 px-5">
      <div className="">
        <h1 className="text-4xl text-gray-500 font-bold">
          Message board frontend
        </h1>
      </div>
      <div className="flex items-center">
        {isAuthenticated && ( // Show links when user is authenticated
          <>
            <Link
              to="/"
              className="text-gray-500 mx-4 hover:text-blue-500"
            >
              Home
            </Link>
            <Link
              to="/add-message"
              className="text-gray-500 mx-4 hover:text-blue-500"
            >
              Add Message
            </Link>
            
          </>
        )}
        <button
          onClick={isAuthenticated ? logout : login}
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        >
          {isAuthenticated ? "Logout" : "Login"}
        </button>
      </div>
    </div>
  );
};

export default Navbar;
