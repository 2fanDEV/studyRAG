import { useEffect, useRef } from "react";

export interface TextAreaProps {
  text: string
  placeholder: string,
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
      <textarea
        ref={textAreaRef}
        onChange={handleChange}
        defaultValue={props.text}
        autoFocus
        className="focus:outline-none w-full resize-none"
        placeholder={props.placeholder}
      />
  );
}
