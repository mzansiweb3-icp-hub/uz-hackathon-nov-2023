import { useEffect, useState } from "react";
import MessageDisplay from "./MessageDisplay";

const Home = ({ isAuthenticated, backendActor }) => {
  const [messages, setMessages] = useState([]);

  useEffect(() => {
    getMessages();
  }, [messages]);

  const getMessages = async () => {
    try {
      const messages = await backendActor.getMessages();
      setMessages(messages);
    } catch (error) {
      console.log("Error on get messages ", error);
    }
  };

  return (
    <div className="bg-gray-900 min-h-screen flex justify-center items-center">
      <div className="w-full max-w-md mx-auto p-6 rounded-lg shadow-md">
        {isAuthenticated ? (
          <div className="bg-slate-300 rounded-lg p-4">
            <MessageDisplay messages={messages} />
          </div>
        ) : (
          <div className="text-center">
            <h1 className="text-4xl font-bold mb-4">
              Welcome! Please log in to continue.
            </h1>
          </div>
        )}
      </div>
    </div>
  );
};

export default Home;
