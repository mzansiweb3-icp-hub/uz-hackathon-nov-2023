import { useNavigate } from "react-router-dom";
import { useState, useEffect } from "react";
import styles from './styles.module.css'

function AddTopics({backendActor}){
  console.log("the backend errors ", backendActor)
  const [topic, setTopic] = useState("");
  const [topics, setTopics] = useState("");
  const [sending, setSending] = useState(false);
  const navigate = useNavigate();

  const sendMessage = async (e) => {
    e.preventDefault();
    try {
      setSending(true);
      const message = {
        topic: topic,
      };
      await backendActor.addTopic(message);
      setTopic("");
      setSending(false);
      navigate('/');
    } catch (error) {
      console.log("Error on send title ", error);
      setSending(false);
  }
  };
    return(
        <div>
            <h1 className="">AddTopics</h1>
            <hr/>
            <fieldset style={{height:"35vh"}}>
                <div className="form-cover">
                    <form method="" action="" style={{height:"35vh"}}>
                        <label>Topics to be used to generate text</label>
                        <textarea 
                            name="topics" 
                            placeholder="eg. myself"
                            value={topic}
                            onChange={(e) => setTopic(e.target.value)}
                            ></textarea><br />
                            {
                          sending ? (
                            <button onClick={sendMessage} disabled={true} >Submiting....</button>
                          ) : (
                            <button onClick={sendMessage} >Submit</button>
                          )
                        }
                    </form>
                </div>
            </fieldset>
        </div>
    )
}
export default AddTopics