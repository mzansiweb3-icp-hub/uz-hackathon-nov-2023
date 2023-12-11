import { useState, useEffect } from "react";
import styles from './styles.module.css'

function ViewStories({backendActor}){
    const [stories, setStories] = useState([])
    console.log("the backend errors ", backendActor)
    const [specificLang, setSpecificLang] = useState("")
    const [getting, setGetting] = useState(false);
    useEffect(() => {
        getLanguagess();
      }, []);

    const getLanguagess = async () => {
        try {
          const messages = await backendActor.getMessages();
          setStories(messages);
          setGetting(true);
        } catch (error) {
          console.log("Error on getting topics ", error);
          setGetting(false);
        }
      };
      const sendMessage = async (e) => {
        e.preventDefault();
        try {
          setGetting(true);
          const messages = await backendActor.getMessagesOnLanguage(specificLang);
          setSpecificLang("");
          setStories(messages)
          setGetting(true);
        } catch (error) {
          console.log("Error on send title ", error);
          setGetting(false);
        }
      };
      const getFile = () => {
        let info = ""
        stories.forEach((story) => {info = info + story.language + "-" + story.language1 + "-" + story.language2 + "\n"})
        const element = document.createElement("a")
        const file = new Blob([info], {
          type:"text/plain; charset=utf-8",
        })
        element.href = URL.createObjectURL(file)
        element.download = "data.txt"
        document.body.appendChild(element)
        element.click()

      }
    return(
        <div>
            <h1>View Stories</h1>
            <hr/>
            <div >
                <fieldset style={{height:"20vh"}}>
                    <form style={{height:"20vh"}}>
                        <label>Language : </label> 
                        <input type="text" name="specificLang" value={specificLang} onChange ={e => setSpecificLang(e.target.value)} />
                        {
                          !getting ? (
                            <button onClick={sendMessage} disabled={true} >Submiting....</button>
                          ) : (
                            <button onClick={sendMessage} >Submit</button>
                          )
                        }
                    </form>
                </fieldset>
            </div>
            <div>
                <h1>Contents</h1>
                <table >
                    <tr>
                        <thead>
                            <th>Language</th>
                            <th>Language 1</th>
                            <th>Language 2</th>
                            <th>Topic</th>
                        </thead>
                    </tr>
                    {
                              getting ? (
                                stories.length > 0 ? (
                                  stories.map((story) => (
                                      <tr>
                                        <tbody>
                                            <td>{story.language}</td>
                                            <td>{story.language1}</td>
                                            <td>{story.language2}</td>
                                            <td>{story.topic}</td>
                                        </tbody>
                                      </tr>
                                  ))
                                ) : (
                                  <tr>
                                        <tbody>
                                            <td>no stories to display yet</td>
                                        </tbody>
                                      </tr>
                                )
                              ) : (
                                <p>Loading....</p>
                              )
                            }
                    
                </table>
                <button onClick={getFile} >Get File</button>
            </div>
            

        </div>
    )
}
export default ViewStories