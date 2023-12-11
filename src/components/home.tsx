import NavBar from "./navBar"
import { useNavigate } from "react-router-dom";
import { useState, useEffect } from "react";
import styles from './style.module.css'

function Home({backendActor}){
    const [languages, setLanguages] = useState([]);
    console.log("the backend errors ", backendActor)
    useEffect(() => {
        getLanguagess();
      }, []);

    const getLanguagess = async () => {
        try {
          const messages = await backendActor.getLanguages();
          setLanguages(messages);
          console.log(messages.language)
          console.log(messages)
          console.log("the backend errors ", backendActor)
        } catch (error) {
          console.log("Error on getting topics ", error);
          console.log("the backend errors ", backendActor)
        }
      };
    
    return(
        <div>
            <div className={styles.languages}>
            {
                                languages.length > 0 ? (
                                    languages.map((lang) => (
                                        <div>
                                            {lang.language}
                                        </div>
                                    )) 
                                ): (
                                    <p> No languages avaliable</p>
                                )
            } 
            </div>
        </div>
    )
}
export default Home
