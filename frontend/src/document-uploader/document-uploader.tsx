import { useState } from "react";
import UploaderModal from "../uploader/uploader-modal";

export default function DocumentUploader() {
  const [minimized, setMinimized] = useState(false);
  const [upload, setUpload] = useState(false);

  let handleClick = (): undefined => {
    setMinimized(!minimized);
    setUpload(false);
  };

  let uploadButton = (): undefined => {
    setUpload(true);
  };

  let uploaderMenu = () => {
    let element = (
      <div className="">
        <button className="" onClick={() => handleClick()}>
          {" "}
          +{" "}
        </button>
      </div>
    );
    if (!minimized) {
      element = (
        <div className="flex flex-col">
          <button className="" onClick={() => handleClick()}>
            -
          </button>
          <button onClick={() => uploadButton()}>Upload</button>
        </div>
      );
    }
    return element;
  };

  return (
    <div>
      <div className="absolute mt-5 mr-5 top-0 right-0">
        <div className="bg-teal-700">{uploaderMenu()}</div>
      </div>
      <div>
        {upload && !minimized ? (
          <div className="fixed flex items-center justify-center bg-black/50">
            <div className="">
              <UploaderModal setUpload={setUpload} />
            </div>
          </div>
        ) : null}
      </div>
    </div>
  );
}
