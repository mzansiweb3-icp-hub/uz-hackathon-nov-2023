import React, { useState, FC } from "react";
import { addArtist } from "./Backend"; // Adjust the path based on your file structure

interface ArtistRegistrationProps {
  onSuccess: () => void; // Function to execute on successful registration
}

const ArtistRegistration: FC<ArtistRegistrationProps> = ({ onSuccess }) => {
  const [fullName, setFullName] = useState<string>("");
  const [pseudoname, setPseudoname] = useState<string>("");
  const [nationalID, setNationalID] = useState<string>("");
  const [dateOfBirth, setDateOfBirth] = useState<string>("");
  const [nationality, setNationality] = useState<string>("");
  const [genre, setGenre] = useState<string>("");
  const [phonenumber, setPhonenumber] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [registering, setRegistering] = useState<boolean>(false);

  const registerArtist = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setRegistering(true);
      const artistPayload = {
        FullName: fullName,
        Pseudoname: pseudoname,
        National_ID: nationalID,
        Date_of_Birth: dateOfBirth,
        Nationality: nationality,
        Genre: genre,
        Phonenumber: phonenumber,
        Email: email,
      };
      await addArtist(artistPayload);
      setFullName("");
      setPseudoname("");
      setNationalID("");
      setDateOfBirth("");
      setNationality("");
      setGenre("");
      setPhonenumber("");
      setEmail("");
      setRegistering(false);
      onSuccess(); // Execute the success callback
    } catch (error) {
      console.log("Error on artist registration", error);
      setRegistering(false);
    }
  };

  return (
    <div className="bg-white rounded-lg p-4 w-full max-w-md mx-auto mt-10 shadow-md">
      <form>
        <div className="mb-4">
          <label htmlFor="fullName" className="block text-gray-700">
            Full Name
          </label>
          <input
            type="text"
            name="fullName"
            value={fullName}
            onChange={(e) => setFullName(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter full name"
          />
        </div>
        {/* Add other input fields for artist registration... */}
        <div className="mb-4">
          <button
            type="submit"
            onClick={registerArtist}
            className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:ring focus:border-blue-300"
          >
            {registering ? "Registering..." : "Register Artist"}
          </button>
        </div>
      </form>
    </div>
  );
};

export default ArtistRegistration;
