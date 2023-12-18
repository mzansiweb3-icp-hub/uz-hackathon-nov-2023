import { FunctionComponent, useState, useEffect } from "react";
import Footer from "../components/Footer";
import Header from "../components/Header";

const Learn: FunctionComponent = () => {
  const [rectangleInputValue, setRectangleInputValue] = useState<string>("");
  useEffect(() => {
    const scrollAnimElements = document.querySelectorAll(
      "[data-animate-on-scroll]"
    );
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting || entry.intersectionRatio > 0) {
            const targetElement = entry.target;
            targetElement.classList.add("animate");
            observer.unobserve(targetElement);
          }
        }
      },
      {
        threshold: 0.15,
      }
    );

    for (let i = 0; i < scrollAnimElements.length; i++) {
      observer.observe(scrollAnimElements[i]);
    }

    return () => {
      for (let i = 0; i < scrollAnimElements.length; i++) {
        observer.unobserve(scrollAnimElements[i]);
      }
    };
  }, []);
  return (
    <div className="relative bg-black1 w-full h-[1434px] overflow-hidden text-left text-[48px] text-dimgray-200 font-montserrat">
      <div className="absolute top-[315px] left-[91.8px] w-[440px] flex flex-row flex-wrap items-center justify-between lg:flex-row lg:gap-[-440px] lg:items-start lg:justify-start md:flex-col md:gap-[-440px] md:items-start md:justify-start">
        <div
          className="relative tracking-[0.2px] leading-[64px] font-extrabold inline-block w-[440px] shrink-0 [&.animate]:animate-[2.9s_ease_1s_1_normal_forwards_fade-in] opacity-[0]"
          data-animate-on-scroll
        >
          <span>{`Become a Master in Blockchain with `}</span>
          <span className="text-whitesmoke-200">OneWeb3</span>
        </div>
        <div
          className="relative text-sm tracking-[0.2px] leading-[20px] inline-block w-[357px] shrink-0 [&.animate]:animate-[2s_ease_2s_1_normal_forwards_fade-in] opacity-[0] text-gray-100"
          data-animate-on-scroll
        >
          <span>{`Empower yourself with Web 3 and Blockchain skills with, access to over `}</span>
          <span className="text-whitesmoke-100">200+</span>
          <span> courses from instituitions and foundations</span>
        </div>
      </div>
      <div className="absolute top-[678px] left-[91.8px] shadow-[-4px_-4px_44px_rgba(0,_0,_0,_0.08)] w-[648px] flex flex-col items-center justify-center">
        <input
          className="[border:none] [outline:none] bg-gray-white self-stretch relative rounded-lg h-[72px]"
          placeholder="Search for course"
          type="text"
          value={rectangleInputValue}
          onChange={(event) => setRectangleInputValue(event.target.value)}
        />
      </div>
      <div className="absolute top-[294px] left-[857.8px] w-[433px] h-[641px] flex items-center justify-center">
        <img
          className="w-full h-full object-contain absolute left-[0px] top-[4px] [transform:scale(1.018)]"
          alt=""
          src="/image-1@2x.png"
        />
      </div>
      <Header />
      <Footer
        propTop="1086px"
        propBackgroundColor="#000"
        propFlexShrink="unset"
      />
      
    </div>
  );
};

export default Learn;
