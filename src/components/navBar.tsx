import { Link } from 'react-router-dom'
import styles from './style.module.css'

function NavBar(){
    return(
        <div className={styles.navBar}>
            <nav>
                <ul>
                    <li><Link to="/"> Home </Link></li>
                    <li><Link to="/createLanguage"> Create Language </Link></li>
                    <li><Link to="/addTopics"> Add Topics </Link></li>
                    <li><Link to="/viewStories"> View Stories </Link></li>
                    <li><Link to="/tellStories"> Tell Stories </Link></li>
                </ul>
            </nav>
        </div>
    )
}
export default NavBar
