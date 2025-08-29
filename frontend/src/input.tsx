import { useState } from "react";
import type { PdfDocument } from "./Pdf";

type UploadedFile = {
	name: string;
	type: string;
	size: number;
	buffer: Uint8Array;
}

export default function FileInput() {
  const [files, setFiles] = useState<File[]>([]);

  let onFileChange = (event: React.ChangeEvent<HTMLInputElement>): void => {
    const files = event.target.files;
    console.log(files);
    if (files) {
      setFiles(Array.from(files)); 
    }
  };

  let handleUpload = async () => {
	console.log(files);
		for (const file of files) {
			const pdf: PdfDocument = {
				id: "123",
				file_name: file.name,
				page_length: 0,
				length: file.size,
				blob: await file.arrayBuffer().then(buffer => Array.from(new Uint8Array(buffer))),
				ty: file.type
			};
			console.log(pdf);
			fetch("http://localhost:8080/pdf/upload", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(pdf)
			})
		}
  }

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
		<button onClick={() => handleUpload()}> Upload all selected files</button>
      </div>
    </div>
  );
}
