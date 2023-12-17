import { FunctionComponent } from "react";
import SideBarsNews from "./SideBarsNews";
/*import {
  createNewsArticle,
  getNewsArticle,
  listNewsArticles,
  updateNewsArticle,
  deleteNewsArticle,
} from '../backend/index';*/


const NewsContainer: FunctionComponent = () => {
  return (
    <div className="absolute top-[172px] left-[0px] w-[1366px] overflow-hidden flex flex-col items-end justify-center text-left text-sm text-darkgray font-montserrat">
      <div className="w-[1364px] overflow-hidden flex flex-col items-start justify-center py-0 px-[68px] box-border">
        <div className="overflow-hidden flex flex-row items-start justify-center gap-[30px]">
          <div className="bg-gray-white w-[670px] overflow-hidden shrink-0 flex flex-row items-start justify-start py-[30px] px-0 box-border">
            <div className="shrink-0 flex flex-col items-center justify-start gap-[24px]">
              <div className="w-[610px] shrink-0 flex flex-row items-start justify-start gap-[2px]">
                <a className="[text-decoration:none] shrink-0 flex flex-row items-center justify-center text-black1">
                  <a className="[text-decoration:none] relative leading-[150%] text-[inherit]">
                    <span>Hackathons</span>
                    <span className="font-montserrat-alternates text-darkgray">{` `}</span>
                  </a>
                </a>
                <a className="[text-decoration:none] relative leading-[150%] text-[inherit] font-montserrat-alternates">
                  <span>
                    <span>{` >`}</span>
                  </span>
                  <span className="text-black1 font-montserrat">
                    <span>{` `}</span>
                    <span>Hackathons</span>
                  </span>
                </a>
                <a className="[text-decoration:none] relative leading-[150%] text-[inherit]">
                  <span>{` >`}</span>
                  <span className="text-black1"> Mzansi Web3</span>
                </a>
              </div>
              <div className="shrink-0 flex flex-col items-start justify-start gap-[16px] text-11xl text-content-main">
                <b className="relative leading-[130%] inline-block w-[610px] h-[67px] shrink-0">
                  Mzansi Web3 Hackathon
                </b>
                <div className="relative text-sm leading-[150%] text-black1 inline-block w-[610px]">
                  10 December 2023, 09:00am
                </div>
                <div className="relative text-base leading-[130%] font-medium inline-block w-[610px]">
                  by Nimakra
                </div>
              </div>
              <div className="w-[670px] h-[381px] flex flex-col items-center justify-end">
                <img
                  className="relative w-[670px] h-[365px] object-cover"
                  alt=""
                  src="/image-25@2x.png"
                />
              </div>
              <div className="relative text-base leading-[130%] text-black inline-block w-[640px]">
                <p className="m-0">
                  The University of Zimbabwe buzzed with excitement as students,
                  developers, and innovators converged for the Mzansi Web3
                  Hackathon. As a participant, I can tell you, it wasn't just
                  about the code. It was about the connections, the shared
                  passion, and the sheer determination to make a difference with
                  Web3.
                </p>
                <p className="m-0">&nbsp;</p>
                <p className="m-0">
                  The hackathon wasn't just about competition; it was about
                  learning, sharing, and supporting each other.
                </p>
                <p className="m-0">&nbsp;</p>
                <p className="m-0">
                  So, if you were looking for an experience that would challenge
                  you, inspire you, and connect you to a community of passionate
                  changemakers, Mzansi’s Web3 Hackathon was the right place. It
                  was more than just a competition; it was a springboard to a
                  future where technology empowers, innovation thrives, and the
                  impossible becomes possible.
                </p>
              </div>
            </div>
          </div>
          <div className="bg-gray-white w-[470px] overflow-hidden shrink-0 flex flex-col items-center justify-start py-[30px] px-0 box-border text-11xl text-content-main">
            <div className="flex flex-col items-start justify-start">
              <b className="relative leading-[130%] inline-block w-[410px]">
                More News
              </b>
              <SideBarsNews />
              <SideBarsNews propTextDecoration="unset" />
              <SideBarsNews propTextDecoration="none" />
              <SideBarsNews propTextDecoration="unset" />
              <SideBarsNews propTextDecoration="unset" />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NewsContainer;
