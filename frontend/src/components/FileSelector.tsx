import { useRef, type ChangeEvent } from "react";
import type { DraggableElement } from "../types/app";

type SelectorProps = {
  name: string;
  draggableElements: React.Dispatch<React.SetStateAction<DraggableElement[]>>;
};

export default function FileSelectorButton(props: SelectorProps) {
  const fileSelectorRef = useRef<HTMLInputElement>(null);

  const handleClick = () => {
    fileSelectorRef.current?.click();
  };

  const handleFileSelect = (event: ChangeEvent<HTMLInputElement>) => {
    let files = event.target.files;
    if (files && files.length > 0) {
      props.draggableElements((prev) => [
        ...prev,
        {
          id: files[0].name,
          position: { x: 0, y: 0 },
        },
      ]);
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
