import { useEffect, useState } from "react";
import useKeyboardShortcut from "../hooks/useKeyboardShortcut";
import getControlOrCommandKey from "../util/os";
import AutoExpandingTextArea from "./AutoExpandingTextArea";
import { useSendQuery } from "../api/embeddings";
import { BsShift } from "react-icons/bs";
import { FaPlus } from "react-icons/fa6";
import { AiOutlineEnter } from "react-icons/ai";

export default function QueryModal(props: any) {
  const osKey = getControlOrCommandKey();
  const showModalCombination = [osKey.technical_id, "k"];
  const closeModalCombination = ["Escape"];
  const { sendQuery } = useSendQuery();
  const { pressedKeys, clearKeys } = useKeyboardShortcut(
    showModalCombination,
    closeModalCombination,
    true
  );
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    const hasShowCombination = showModalCombination.every((key) =>
      pressedKeys.has(key)
    );

    const hasCloseCombination = closeModalCombination.every((key) =>
      pressedKeys.has(key)
    );

    if (hasShowCombination) {
      setShowModal((prev) => {
        return true;
      });
      clearKeys();
    } else if (hasCloseCombination) {
      setShowModal((prev) => {
        return false;
      });
      clearKeys();
    }
  }, [pressedKeys, showModalCombination]);

  const inputCallback = async (input: string) => {
    await sendQuery({ text: input });
    setShowModal((prev) => {
      return false;
    });
  };

  let dialog = <div className="text-white"> </div>;
  if (showModal) {
    dialog = (
      <div className="flex flex-col justify-center text-white absolute bg-transparent w-full h-full">
        <div className="self-center">
          <AutoExpandingTextArea
            inputCallback={inputCallback}
          ></AutoExpandingTextArea>
        </div>
        <div className="self-center opacity-0 animate-fadeIn0_20 mt-3 ml-90 gap-1">
          <div className="flex gap-1 text-xs border-1 rounded-2xl p-1 border-teal-500">
            <BsShift />
            <FaPlus />
            <AiOutlineEnter />{" "}
          </div>
        </div>
      </div>
    );
  }

  return dialog;
}
