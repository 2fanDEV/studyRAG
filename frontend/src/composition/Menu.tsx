import LoginButton from "@/components/LoginButton";
import ModelProvider from "@/components/ModelProvider";
import UploadButton from "@/components/UploadButton";

export function Menu() {
  return (
    <div className="grid w-full grid-cols-2 absolute">
        <div>
            <ModelProvider/>
        </div>
      <div className="w-full p-5 gap-5 flex justify-end z-10">
        <UploadButton />
        <div className="mt-1.5">
          <LoginButton />
        </div>
      </div>
    </div>
  );
}
