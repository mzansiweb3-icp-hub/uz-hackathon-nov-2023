import { Link } from 'react-router-dom';

const Navbar = ({ isAuthenticated, login, logout }) => {
  return (
    <nav className="bg-gray-800 text-white p-4">
      <div className="flex justify-between items-center">
        <h1 className="text-xl font-bold"> <Link to={"/"}>Micro-Exchange</Link></h1>
        <div>
          {!isAuthenticated ? (
            <button className="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded" onClick={login}>
              Login
            </button>
          ) : (
            <div className="flex">
              <Link to="/borrow" className="mx-4 text-blue-300 hover:text-blue-100 text-lg">Borrow</Link>
              <Link to="/lend" className="mx-4 text-green-300 hover:text-green-100 text-lg">Lend</Link>
              <button className="bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded" onClick={logout}>
                Logout
              </button>
            </div>
          )}
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
