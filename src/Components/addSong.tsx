import React, { useState, FC } from "react";
interface SongCreationProps {
  onSuccess: () => void; // Function to execute on successful song creation
}

const SongCreation: FC<SongCreationProps> = ({ onSuccess }) => {
  const [title, setTitle] = useState<string>("");
  const [composer, setComposer] = useState<string>("");
  const [artistId, setArtistId] = useState<string>("");
  const [creating, setCreating] = useState<boolean>(false);

  const createSong = async (e: React.FormEvent) => {
    e.preventDefault();bd3sg-teaaa-aaaaa-qaaba-cai
    try {
      setCreating(true);
      const songPayload = {
        Title: title,
        Composer: composer,
        ArtistId: artistId,
      };
      await addSong(songPayload);
      setTitle("");
      setComposer("");
      setArtistId("");
      setCreating(false);
      onSuccess(); // Execute the success callback
    } catch (error) {
      console.log("Error on song creation", error);
      setCreating(false);
    }
  };

  return (
    <div className="bg-white rounded-lg p-4 w-full max-w-md mx-auto mt-10 shadow-md">
      <form>
        <div className="mb-4">
          <label htmlFor="title" className="block text-gray-700">
            Title
          </label>
          <input
            type="text"
            name="title"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter title"
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
          <label htmlFor="artistId" className="block text-gray-700">
            Artist ID
          </label>
          <input
            type="text"
            name="artistId"
            value={artistId}
            onChange={(e) => setArtistId(e.target.value)}
            className="w-full mt-1 p-3 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-500"
            placeholder="Enter artist ID"
          />
        </div>
        <div className="mb-4">
          <button
            type="submit"
            onClick={createSong}
            className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:ring focus:border-blue-300"
          >
            {creating ? "Creating..." : "Create Song"}
          </button>
        </div>
      </form>
    </div>
  );
};

export default SongCreation;
