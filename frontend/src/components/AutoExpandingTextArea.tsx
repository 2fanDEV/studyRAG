import { useEffect, useRef, useState } from "react";
import useKeyboardShortcut from "../hooks/useKeyboardShortcut";

export interface TextAreaProps {
  inputCallback: (input: string) => void;
}

export default function AutoExpandingTextArea(props: TextAreaProps) {
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const closeInputShortcut = ["Shift", "Enter"];
  const { pressedKeys, clearKeys } = useKeyboardShortcut(
    closeInputShortcut,
    [],
    false
  );

  const adjustHeight = () => {
    if (textAreaRef.current) {
      textAreaRef.current.style.height = "auto";
      textAreaRef.current.style.height = `${textAreaRef.current.scrollHeight}px`;
    }
  };

  useEffect(() => {
    if (closeInputShortcut.every((key) => pressedKeys.has(key))) {
      if (textAreaRef.current) {
        props.inputCallback(textAreaRef.current.value);
        clearKeys();
      }
    }

    adjustHeight();
  }, [pressedKeys, closeInputShortcut]);

  const handleChange = (e: any) => {
    adjustHeight();
  };

  return (
    <div
      className="text-white 
    rounded-2xl 
    p-4
    w-md
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
        autoFocus
        className="focus:outline-none text-xs w-full resize-none"
        placeholder="Place your query here"
      />
    </div>
  );
}
