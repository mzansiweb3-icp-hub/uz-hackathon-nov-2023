import { FunctionComponent } from "react";
import NewsContainer from "../components/NewsContainer";
import Header from "../components/Header";
import Footer from "../components/Footer";

const News: FunctionComponent = () => {
  return (
    <div className="relative bg-black w-full h-[1603px] overflow-y-auto">
      <img
        className="absolute top-[-10px] left-[-411px] w-[1850px] h-[1534px] object-cover hidden"
        alt=""
        src="/image-21@2x.png"
      />
      <NewsContainer />
      <Header />
      <Footer
        propTop="1312px"
        propBackgroundColor="#060606"
        propFlexShrink="0"
      />
    </div>
  );
};

export default News;
