import React, { useState, useEffect, FC } from "react";
import { getArtistDashboard } from "./Backend"; // Adjust the path based on your file structure

interface DashboardViewProps {
  artistId: string;
}

const DashboardView: FC<DashboardViewProps> = ({ artistId }) => {
  const [songs, setSongs] = useState<any[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    const fetchDashboard = async () => {
      try {
        const dashboardData = await getArtistDashboard(artistId);
        setSongs(dashboardData);
        setLoading(false);
      } catch (error) {
        console.error("Error fetching artist dashboard", error);
        setLoading(false);
      }
    };

    fetchDashboard();
  }, [artistId]);

  return (
    <div className="bg-white rounded-lg p-4 w-full max-w-md mx-auto mt-10 shadow-md">
      <h2 className="text-2xl font-bold mb-4">Artist Dashboard</h2>
      {loading ? (
        <p>Loading dashboard...</p>
      ) : (
        <ul>
          {songs.map((song) => (
            <li key={song.id} className="mb-2">
              <strong>Title:</strong> {song.Title}, <strong>Composer:</strong>{" "}
              {song.Composer}, <strong>Play Count:</strong> {song.PlayCount}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default DashboardView;
