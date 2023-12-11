import { useNavigate } from "react-router-dom";
import { useState, useEffect } from "react";
import styles from './styles.module.css'

function TellStories({backendActor}){
    
    const [language1, setLanguage1] = useState("");
    const [language2, setLanguage2] = useState("");
    const [language, setLanguage] = useState("");
    const [topic, setTopic] = useState("");
    const [comment, setComment] = useState("");
    const [email, setEmail] = useState("");
    const [languages, setLanguages] = useState([]);
    const [topics, setTopics] = useState([]);
    const [sending, setSending] = useState(false);
    const navigate = useNavigate();
    const sendMessage = async (e) => {
        e.preventDefault();
        try {
          setSending(true);
          if(email==""){
            setEmail("none@gmail.com")
          }
          const message = {
            language1: language1,
            language: language,
            language2: language2,
            comment: comment,
            email: email,
            topic: topic,
          };
          await backendActor.addMessage(message);
          setEmail("");
          setComment("");
          setLanguage("");
          setLanguage1("");
          setLanguage2("");
          setTopic("");
          console.log("the backend errors ", backendActor)
          setSending(false);
          navigate('/viewStories');
        } catch (error) {
          console.log("Error on send title ", error);
          setSending(false);
        }
      };

      useEffect(() => {
        getLanguagesAndTopics();
      }, []);

    const getLanguagesAndTopics = async () => {
        try {
          const topics = await backendActor.getTopics();
          const languages = await backendActor.getLanguages();
          setTopics(topics);
          setLanguages(languages);
          
        } catch (error) {
          console.log("Error on getting topics ", error);
        }
      };
        console.log("the topics are", topics)
        console.log("the languages", languages)
    return(
        <div>
            <h1 className="">Tell Stories</h1>
            <hr/>
            <fieldset style={{height:"100vh"}}>
                <div className="form-cover">
                    <form method="" action="" style={{height:"100vh"}}>
                        <label>Select Language </label>
                      <select name="language" value={language} onChange={e => setLanguage(e.target.value) }>
                            <option> english-shona </option>
                            
                            {
                              languages.length > 0 ? (
                                languages.map((language) => (
                                  <option>{ language.language }</option>
                                ))
                              ) : (
                                <option> null </option>
                              )
                            }
                        </select><br />
                        <label>Select Topic </label>
                        <select name="topic" value={topic} onChange={e => setTopic(e.target.value) }>
                        {
                              topics.length > 0 ? (
                                topics.map((topic) => (
                                  <option>{ topic.topic }</option>
                                ))
                              ) : (
                                <option> null </option>
                              )
                            }
                        </select><br />
                        <label>Email </label>
                        <input type="email" name ="email" value={email} onChange={(e) => setEmail(e.target.value)} placeholder="eg. language@gmail.com" /><br />
                        <label>Language 1 </label>
                        <textarea name="language1" value={language1} onChange={(e) => setLanguage1(e.target.value)} placeholder="eg. i get home at noon"></textarea><br />
                        <label>Language 1 </label>
                        <textarea name="language2" value={language2} onChange={(e) => setLanguage2(e.target.value)} placeholder="eg. ndasvika kumba masikati"></textarea><br />
                        <label>Comment </label>
                        <textarea name="comment" value={comment} onChange={(e) => setComment(e.target.value)} placeholder="give us your view"></textarea><br />
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
export default TellStories