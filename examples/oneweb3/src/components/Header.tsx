import { FunctionComponent } from "react";
import { useNavigate } from "react-router-dom";

const Header: FunctionComponent = () => {
  const navigate = useNavigate();
  return (
    <header className="absolute top-[0px] left-[0px] w-[1368px] overflow-hidden flex flex-col items-end justify-center">
      <div className="bg-black1 w-[1366px] h-[89px] flex flex-col items-end justify-end py-[5px] px-[34px] box-border">
        <div className="w-[899px] h-[67px] flex flex-row items-center justify-end gap-[53px]">
          <div className="w-[656px] h-6 flex flex-row items-center justify-start gap-[24px]">
            <button onClick={() => navigate("/")} className="cursor-pointer [border:none] p-0 bg-[transparent] relative text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-16 h-6 shrink-0">
              Home
            </button>
            <button onClick={() => navigate("/learn")} className="cursor-pointer [border:none] p-0 bg-[transparent] relative text-lg font-semibold font-raleway text-gray-white text-left inline-block w-[53px] h-[23px] shrink-0">
              Learn
            </button>
            <button onClick={() => navigate("/news")} className="cursor-pointer [border:none] p-0 bg-[transparent] relative text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[70px] h-[23px] shrink-0">
              News
            </button>
            <button onClick={() => navigate("/build")} className="cursor-pointer [border:none] p-0 bg-[transparent] relative text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[65px] h-[23px] shrink-0">
              Build
            </button>
            <button onClick={() => navigate("/communities")} className="cursor-pointer [border:none] p-0 bg-[transparent] relative text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[214px] h-[23px] shrink-0">
              Discover communities
            </button>
            <button onClick={() => navigate("/invest")} className="cursor-pointer [border:none] p-0 bg-[transparent] relative text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[70px] h-[23px] shrink-0">
              Invest
            </button>
          </div>
          <button className="cursor-pointer [border:none] p-5 bg-royalblue rounded-31xl w-[190px] h-[67px] flex flex-row items-center justify-center box-border">
            <div className="relative text-lg font-semibold font-montserrat text-gray-white text-left">
              Create Profile
            </div>
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
