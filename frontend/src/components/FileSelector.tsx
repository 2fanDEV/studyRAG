import { useRef, type ChangeEvent } from "react";
import type { RAGDraggableElement } from "./DraggableElement";
import { FaPlus } from "react-icons/fa";
type SelectorProps = {
  name: string;
  onUpload: (element: RAGDraggableElement, file: File) => void;
};

export default function FileSelectorButton(props: SelectorProps) {
  const fileSelectorRef = useRef<HTMLInputElement>(null);
  const handleClick = () => {
    fileSelectorRef.current?.click();
  };

  const handleFileSelect = (event: ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      let file_name = files[0].name;
      props.onUpload(
        {
          position: { x: Math.random() * 100, y: Math.random() * 100 },
          name: file_name,
          ty: file_name.split(".").pop()?.toUpperCase(),
          len: files[0].size,
        },
        files[0]
      );
    }
  };

  return (
    <div className="">
      <button
        className="bg-teal-400 text-white font-bold
          	border-transparent hover:border-teal-400 border-2
	         	transition-colors ease-in-out duration-500
	        hover:bg-black rounded-full text-xs w-7 h-7 cursor-pointer"
        onClick={handleClick}
      >
      <FaPlus className="ml-1.5"/>
      </button>
      <input
        type="file"
        ref={fileSelectorRef}
        onChange={handleFileSelect}
        style={{ display: "none" }}
      />
    </div>
  );
}
