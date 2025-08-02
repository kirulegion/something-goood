"use cleint";
import Image from "next/image";

const Signup = () => {
  return (
    <div className="h-lvh w-full flex items-center justify-center">
      <div className="border-4 border-white rounded-2xl w-3/4 h-[800px] flex">
        <div className="w-1/2 bg-black rounded-l-xl relative">
          <Image
            src="/login.png"
            alt="logo"
            fill
            className="object-cover rounded-l-md"
          />
        </div>
        <div className="flex flex-col w-1/2  rounded-r-md">
          <h1>Welcome back</h1>
          <p>Login to acesss your account</p>
          <form>
            <label></label>
          </form>
        </div>
      </div>
    </div>
  );
};

export default Signup;
