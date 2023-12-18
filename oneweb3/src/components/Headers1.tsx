import { FunctionComponent } from "react";
import { useNavigate } from "react-router-dom";

const Headers1: FunctionComponent = () => {
  const navigate = useNavigate();
  return (
    <header className="absolute top-[26px] left-[434px] bg-black1 w-[899px] h-[67px]">
      <div className="absolute top-[0px] left-[0px] w-[899px] h-[67px]">
        <div className="absolute top-[20px] left-[0px] w-[654px] h-6">
          <button className="cursor-pointer [border:none] p-0 bg-[transparent] absolute bottom-[0px] left-[0px] text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-16 h-6">
            Home
          </button>
          <button onClick={() => navigate("/learn")} className="cursor-pointer [border:none] p-0 bg-[transparent] absolute top-[1px] left-[88px] text-lg font-semibold font-raleway text-gray-white text-left inline-block w-[53px] h-[23px]">
            Learn
          </button>
          <button  onClick={() => navigate("/news")}
          className="cursor-pointer [border:none] p-0 bg-[transparent] absolute top-[1px] left-[165px] text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[70px] h-[23px]">
            News
          </button>
          <button className="cursor-pointer [border:none] p-0 bg-[transparent] absolute top-[1px] left-[259px] text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[65px] h-[23px]">
            Build
          </button>
          <button 
          className="cursor-pointer [border:none] p-0 bg-[transparent] absolute top-[1px] left-[586px] text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[70px] h-[23px]">
            Invest
          </button>
          <button className="cursor-pointer [border:none] p-0 bg-[transparent] absolute top-[1px] left-[348px] text-lg font-semibold font-montserrat text-gray-white text-left inline-block w-[214px] h-[23px]">
            Discover communities
          </button>
        </div>
        <button className="cursor-pointer [border:none] p-5 bg-royalblue absolute top-[0px] left-[709px] rounded-31xl w-[190px] h-[67px] flex flex-row items-center justify-center box-border">
          <div className="relative text-lg font-semibold font-montserrat text-gray-white text-left">
            Create Profile
          </div>
        </button>
      </div>
    </header>
  );
};

export default Headers1;
