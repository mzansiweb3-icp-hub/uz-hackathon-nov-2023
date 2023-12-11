import { useNavigate } from "react-router-dom";
import { useState, useEffect } from "react";
import styles from './styles.module.css'

function CreateLanguage({backendActor}){
  const [language, setLanguage] = useState("");
  
  const [sending, setSending] = useState(false);
  const [topic, setTopic] = useState("");
  const navigate = useNavigate();
  const sendMessage = async (e) => {
    e.preventDefault();
    try {
      setSending(true);
      const message = {
        language: language,
      };
      const topics = {
        topic: topic,
      };
      await backendActor.addLanguage(message);
      await backendActor.addTopic(topics);
      setTopic("");
      setLanguage("");
      setSending(false);
      console.log("the backend errors ", backendActor)
      navigate('/');
    } catch (error) {
      console.log("Error on send title ", error);
      setSending(false);
    }
  };
    return(
        <div className="cover">
            <h1 className="h-cover">Create language</h1>
            <hr/>
            <fieldset style={{height:"45vh"}}>
                <div className="form-cover">
                    <form method="" action="" style={{height:"45vh"}}>
                        <label>Two language to be interpreted</label>
                        <input type="text" 
                            name="language" 
                            value={language}
                            onChange={(e) => setLanguage(e.target.value)} 
                            placeholder="eg. english-shona" /><br />
                          <label>Topics to be used to generate text</label>
                          <textarea 
                            name="topics" 
                            placeholder="eg. myself"
                            value={topic}
                            onChange={(e) => setTopic(e.target.value)}
                            ></textarea><br />
                        {/* <button type="submit" name="submit" onClick={sendMessage}>Submit</button> */}
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
export default CreateLanguage