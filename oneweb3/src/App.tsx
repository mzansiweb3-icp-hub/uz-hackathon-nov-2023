import { useEffect } from "react";
import {
  Routes,
  Route,
  useNavigationType,
  useLocation,
} from "react-router-dom";
import OneWeb3Main from "./pages/OneWeb3Main";
import Invest from "./pages/Invest";
import Communities from "./pages/Communities";
import News from "./pages/News";
import Learn from "./pages/Learn";
import Build from "./pages/Build";

function App() {
  const action = useNavigationType();
  const location = useLocation();
  const pathname = location.pathname;

  useEffect(() => {
    if (action !== "POP") {
      window.scrollTo(0, 0);
    }
  }, [action, pathname]);

  useEffect(() => {
    let title = "";
    let metaDescription = "";

    switch (pathname) {
      case "/":
        title = "";
        metaDescription = "";
        break;
      case "/invest":
        title = "";
        metaDescription = "";
        break;
      case "/communities":
        title = "";
        metaDescription = "";
        break;
      case "/news":
        title = "";
        metaDescription = "";
        break;
      case "/learn":
        title = "";
        metaDescription = "";
        break;
    }

    if (title) {
      document.title = title;
    }

    if (metaDescription) {
      const metaDescriptionTag: HTMLMetaElement | null = document.querySelector(
        'head > meta[name="description"]'
      );
      if (metaDescriptionTag) {
        metaDescriptionTag.content = metaDescription;
      }
    }
  }, [pathname]);

  return (
    <Routes>
      <Route path="/" element={<OneWeb3Main />} />
      <Route path="/invest" element={<Invest />} />
      <Route path="/communities" element={<Communities />} />
      <Route path="/news" element={<News />} />
      <Route path="/build" element={<Build />} />
      <Route path="/learn" element={<Learn />} />
    </Routes>
  );
}
export default App;
