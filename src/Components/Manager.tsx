import React, { useState, FC } from "react"; 

interface ManagerRegistrationProps {
  onSuccess: () => void; // Function to execute on successful registration
}

const ManagerRegistration: FC<ManagerRegistrationProps> = ({ onSuccess }) => {
  const [name, setName] = useState<string>("");
  const [phonenumber, setPhonenumber] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [registering, setRegistering] = useState<boolean>(false);

  const registerManager = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setRegistering(true);
      const managerPayload = {
        Name: name,
        Phonenumber: phonenumber,
        Email: email,
      };
      await addManager(managerPayload);
      setName("");
      setPhonenumber("");
      setEmail("");
      setRegistering(false);
      onSuccess(); // Execute the success callback
    } catch (error) {
      console.log("Error on manager registration", error);
      setRegistering(false);
    }
  };

  return (
    <div className="bg-white rounded-lg p-4 w-full max-w-md mx-auto mt-10 shadow-md">
      <form>
        <div className="mb-4">
          <label htmlFor="name" className="block text-gray-700">
            Name
          </label>
          <input
            type="text"
            name="name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter name"
          />
        </div>
        <div className="mb-4">
          <label htmlFor="phonenumber" className="block text-gray-700">
            Phone Number
          </label>
          <input
            type="text"
            name="phonenumber"
            value={phonenumber}
            onChange={(e) => setPhonenumber(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter phone number"
          />
        </div>
        <div className="mb-4">
          <label htmlFor="email" className="block text-gray-700">
            Email
          </label>
          <input
            type="text"
            name="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter email"
          />
        </div>
        <div className="mb-4">
          <button
            type="submit"
            onClick={registerManager}
            className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:ring focus:border-blue-300"
          >
            {registering ? "Registering..." : "Register Manager"}
          </button>
        </div>
      </form>
    </div>
  );
};

export default ManagerRegistration;
