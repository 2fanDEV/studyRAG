import { useCallback, useEffect, useState } from "react";

type KeySet = Set<string>;

export default function useKeyboardShortcut(targetKeys: string[]) {
  const [keys, setKeys] = useState<KeySet>(new Set());
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (targetKeys.includes(event.key)) {
        event.preventDefault();
        setKeys((prevKeys) => new Set(prevKeys).add(event.key));
      }
    },
    [targetKeys]
  );

  const handleKeyUp = useCallback(
    (event: KeyboardEvent) => {
      if (targetKeys.includes(event.key)) {
        setKeys((prevKeys) => {
          const newKeys = new Set(prevKeys);
          newKeys.delete(event.key);
          return newKeys;
        });
      }
    },
    [targetKeys]
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [handleKeyDown, handleKeyUp]);

  return keys;
}
