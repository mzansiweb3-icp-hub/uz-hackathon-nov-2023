import React, { useState, FC } from "react";

interface AddLogsheetProps {
  onSuccess: () => void; // Function to execute on successful log sheet addition
}

const AddLogsheet: FC<AddLogsheetProps> = ({ onSuccess }) => {
  const [createdBy, setCreatedBy] = useState<string>("");
  const [songTitle, setSongTitle] = useState<string>("");
  const [numberOfPlays, setNumberOfPlays] = useState<number>(0);
  const [composer, setComposer] = useState<string>("");
  const [adding, setAdding] = useState<boolean>(false);

  const addLogsheets = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setAdding(true);
      const logsheetPayload = {
        createdBy: createdBy,
        SongTitle: songTitle,
        NumberOfPlays: BigInt(numberOfPlays),
        Composer: composer,
      };
      await addLogsheet(logsheetPayload);
      setCreatedBy("");
      setSongTitle("");
      setNumberOfPlays(0);
      setComposer("");
      setAdding(false);
      onSuccess(); // Execute the success callback
    } catch (error) {
      console.error("Error on logsheet addition", error);
      setAdding(false);
    }
  };

  return (
    <div className="bg-white rounded-lg p-4 w-full max-w-md mx-auto mt-10 shadow-md">
      <form>
        <div className="mb-4">
          <label htmlFor="createdBy" className="block text-gray-700">
            Created By
          </label>
          <input
            type="text"
            name="createdBy"
            value={createdBy}
            onChange={(e) => setCreatedBy(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter creator's name"
          />
        </div>
        <div className="mb-4">
          <label htmlFor="songTitle" className="block text-gray-700">
            Song Title
          </label>
          <input
            type="text"
            name="songTitle"
            value={songTitle}
            onChange={(e) => setSongTitle(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter song title"
          />
        </div>
        <div className="mb-4">
          <label htmlFor="numberOfPlays" className="block text-gray-700">
            Number of Plays
          </label>
          <input
            type="number"
            name="numberOfPlays"
            value={numberOfPlays}
            onChange={(e) => setNumberOfPlays(Number(e.target.value))}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter number of plays"
          />
        </div>
        <div className="mb-4">
          <label htmlFor="composer" className="block text-gray-700">
            Composer
          </label>
          <input
            type="text"
            name="composer"
            value={composer}
            onChange={(e) => setComposer(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter composer"
          />
        </div>
        <div className="mb-4">
          <button
            type="submit"
            onClick={addLogsheets}
            className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:ring focus:border-blue-300"
          >
            {adding ? "Adding..." : "Add Logsheet"}
          </button>
        </div>
      </form>
    </div>
  );
};

export default AddLogsheet;
