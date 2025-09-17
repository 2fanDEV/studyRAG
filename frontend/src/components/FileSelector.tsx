import { useRef, type ChangeEvent } from "react";
import type { RAGDraggableElement } from "./DraggableElement";
type SelectorProps = {
  name: string;
  onUpload: (element: RAGDraggableElement) => void;
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
          ty: file_name.split(".").pop(),
          path: URL.createObjectURL(files[0]),
          len: files[0].size,
        }
      );
    }
  };

  return (
    <div className="">
      <button
        className="bg-[#319cce] text-white font-bold
          	border-transparent hover:border-teal-400 border-2
	         	transition-colors ease-in-out duration-500
	        hover:bg-[#23759b] rounded-full text-xs w-7 h-7 cursor-pointer"
        onClick={handleClick}
      >
        <a className="fa-solid fa-plus"></a>
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
