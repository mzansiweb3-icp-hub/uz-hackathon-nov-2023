import { FunctionComponent, useMemo, type CSSProperties } from "react";

type FooterType = {
  /** Style props */
  propTop?: CSSProperties["top"];
  propBackgroundColor?: CSSProperties["backgroundColor"];
  propFlexShrink?: CSSProperties["flexShrink"];
};

const Footer: FunctionComponent<FooterType> = ({
  propTop,
  propBackgroundColor,
  propFlexShrink,
}) => {
  const footerStyle: CSSProperties = useMemo(() => {
    return {
      top: propTop,
      backgroundColor: propBackgroundColor,
      flexShrink: propFlexShrink,
    };
  }, [propTop, propBackgroundColor, propFlexShrink]);

  return (
    <footer
      className="absolute top-[1312px] left-[0px] bg-black w-[1364px] shrink-0 flex flex-row items-start justify-between py-[31px] px-[126px] box-border text-left text-xl text-gray-white font-montserrat md:flex-col md:gap-[107.5px] md:items-start md:justify-start"
      style={footerStyle}
    >
      <div className="w-28 h-[229px] flex flex-col items-start justify-start gap-[26px]">
        <b className="relative text-5xl text-center">Learn</b>
        <div className="relative">Courses</div>
        <div className="relative">DeFi</div>
        <div className="relative">NFTs</div>
        <div className="relative">Blockchain</div>
      </div>
      <div className="w-[106px] h-44 flex flex-col items-start justify-start gap-[25px]">
        <b className="relative text-5xl inline-block text-center w-[70px] h-[29px] shrink-0">
          News
        </b>
        <div className="relative inline-block w-[91px] h-6 shrink-0">
          Trending
        </div>
        <div className="relative inline-block w-[106px] h-6 shrink-0">
          Hot Topics
        </div>
        <div className="relative inline-block w-[35px] h-6 shrink-0">ICP</div>
      </div>
      <div className="w-[164px] h-44 flex flex-col items-start justify-start py-0 pr-0 pl-0.5 box-border gap-[25px]">
        <b className="relative text-5xl text-center">Build</b>
        <div className="relative">DApps</div>
        <div className="relative">Smart Contracts</div>
        <div className="relative">DAOs</div>
      </div>
      <div className="w-[170px] h-[229px] flex flex-col items-start justify-start gap-[26px]">
        <b className="relative text-5xl text-center">Communities</b>
        <div className="relative">DFinity</div>
        <div className="relative">Mzansi Web3</div>
        <div className="relative">Dacade</div>
        <div className="relative">Bitcoin</div>
      </div>
      <div className="w-[130px] h-[72px] flex flex-col items-start justify-start gap-[19px] text-center text-5xl">
        <b className="relative">Resources</b>
        <div className="relative text-xl text-left">About</div>
      </div>
    </footer>
  );
};

export default Footer;
