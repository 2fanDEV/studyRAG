import { useRef } from "react";

export default function AutoExpandingTextArea(props: any) {
  const textAreaRef = useRef(null);

    const adjustHeight = () => {
        if(textAreaRef.current) {
            textAreaRef.current.style.height = "auto";
        }
    }


  return (
    <div
    ref={textAreaRef}
      className="text-white 
    rounded-2xl 
    absolute
    p-9 
    backdrop-blur-xxs 
    border-white bg-[linear-gradient(to_top,#008080,transparent)]"
    >
      <textarea
        className="text-xs resize-none"
        placeholder="Place your query here"
      />
    </div>
  );
}
