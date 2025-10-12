import { useEffect, useRef } from "react";

export interface TextAreaProps {
  text: string
  inputCallback: (input: string) => void;
}

export default function AutoExpandingTextArea(props: TextAreaProps) {
  const textAreaRef = useRef<HTMLTextAreaElement>(null);

  const adjustHeight = () => {
    if (textAreaRef.current) {
      textAreaRef.current.style.height = "auto";
      textAreaRef.current.style.height = `${textAreaRef.current.scrollHeight}px`;
      setCursorAtLength();
    }
  };

  const setCursorAtLength = () => {
    if(textAreaRef.current) {
      let length = textAreaRef.current.value.length;
      textAreaRef.current.setSelectionRange(length, length);
    }
  }

  useEffect(() => {
    adjustHeight();
  }, []);

  const handleChange = (e: any) => {
    if (textAreaRef.current) {
      props.inputCallback(textAreaRef.current.value);
    }
    adjustHeight();
  };

  return (
    <div
      className="text-white
    rounded-2xl 
    p-4
    w-xl
    drop-shadow-white
    drop-shadow-xs
    mr-5
    animate-fadeIn0_20
    border-white bg-[linear-gradient(to_top,#00808077,transparent)]
    backdrop-blur-[1px]"
    >
      <textarea
        ref={textAreaRef}
        onChange={handleChange}
        defaultValue={props.text}
        autoFocus
        className="focus:outline-none text-xl w-full resize-none"
        placeholder="Place your query here"
      />
    </div>
  );
}
