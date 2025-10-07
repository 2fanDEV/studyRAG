import { useEffect, useState } from "react";
import useKeyboardShortcut from "../hooks/useKeyboardShortcut";
import getControlOrCommandKey from "../util/os";
import AutoExpandingTextArea from "./AutoExpandingTextArea";
import { useSendQuery } from "../api/embeddings";
import { BsShift } from "react-icons/bs";
import { FaPlus } from "react-icons/fa6";
import { AiOutlineEnter } from "react-icons/ai";
import Models from "./Models";
import type { Model } from "@/types/models.d";

export interface QueryProps {
  selectedModel: (model: Model) => void,
  models: Model[]
}


export default function QueryModal(props: QueryProps) {
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
      <div className="w-full h-full absolute flex justify-center ">
      <div className="flex flex-col justify-center text-white bg-transparent w-md h-full">
        <div className="self-center">
          <AutoExpandingTextArea
            inputCallback={inputCallback}
          ></AutoExpandingTextArea>
        </div>
        <div className="grid grid-cols-2 opacity-0 animate-fadeIn0_20 mt-3 mr-5">
          <div>
      <Models selectedModel={ (model) => { console.log(model)}} provider="GitHub"/>
        </div>
          <div className="flex gap-1 text-xs border-2 w-15 justify-self-end rounded-2xl p-1 h-6 border-teal-500">
            <BsShift />
            <FaPlus />
            <AiOutlineEnter />{" "}
          </div>
        </div>
      </div>
      </div>
    );
  }

  return dialog;
}
