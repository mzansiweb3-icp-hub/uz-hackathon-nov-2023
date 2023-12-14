import React, { useState, useEffect, FC } from "react";

interface ViewSongsProps {}

const ViewSongs: FC<ViewSongsProps> = () => {
  const [songs, setSongs] = useState<any[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    const fetchSongs = async () => {
      try {
        const songsData = await getAllSongs();
        setSongs(songsData);
        setLoading(false);
      } catch (error) {
        console.error("Error fetching songs", error);
        setLoading(false);
      }
    };

    fetchSongs();
  }, []);

  return (
    <div className="bg-white rounded-lg p-4 w-full max-w-md mx-auto mt-10 shadow-md">
      <h2 className="text-2xl font-bold mb-4">Songs List</h2>
      {loading ? (
        <p>Loading songs...</p>
      ) : (
        <ul>
          {songs.map((song) => (
            <li key={song.id} className="mb-2">
              <strong>Title:</strong> {song.Title}, <strong>Composer:</strong>{" "}
              {song.Composer}, <strong>Artist ID:</strong> {song.ArtistId}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default ViewSongs;
