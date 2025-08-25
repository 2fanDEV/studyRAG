import { useState } from "react";

export default function FileInput() {
  const [files, setFiles] = useState<File[]>([]);

  let onFileChange = (event: React.ChangeEvent<HTMLInputElement>): void => {
    const files = event.target.files;
    console.log(files);
    if (files) {
      setFiles(Array.from(files));
    }
  };

  let fileList = (files.map((file, index) => {
    return (
      <li key={index}>
        {file.name} - {file.size/10000} MB
      </li>
    );
  }));

  return (
    <div className="relative ml-5 mr-5 w-2xs self-center">
      <div>
        <label
          htmlFor="fileUpload"
          className="cursor-pointer px-4 py-2 rounded-md bg-blue-600 text-white hover:bg-blue-700 font-medium"
        >
          Select files
        </label>
        <input id="fileUpload" type="file" className="hidden" onChange={onFileChange} multiple />
		{files.length > 0 && (
			<div className="mt-4">
				<h3 className="font-medium mb-2">Selected Files:</h3>
				<ul>{fileList}</ul>
			</div>
		)}
      </div>
    </div>
  );
}
