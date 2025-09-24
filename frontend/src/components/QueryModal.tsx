import { useEffect, useState } from "react";
import useKeyboardShortcut from "../hooks/useKeyboardShortcut";
import getControlOrCommandKey from "../util/os";

export default function QueryModal(props: any) {
  const osKey = getControlOrCommandKey();
  const showModalCombination = [osKey.technical_id, "k"];
  const closeModalCombination = ["Escape"];
  const pressedKeys = useKeyboardShortcut(
    showModalCombination,
    closeModalCombination
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
    } else if (hasCloseCombination) {
      setShowModal((prev) => {
        return false;
      });
    }
  }, [pressedKeys, showModalCombination]);

  let dialog = <div className="text-white"> 1 </div>;
  if (showModal) {
    dialog = <div className="text-white"> 
    <input type="text" /> </div>;
  }

  return dialog;
}
