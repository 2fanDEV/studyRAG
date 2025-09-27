import { useCallback, useEffect, useState } from "react";

export type KeySet = Set<string>;

export type KeyboardShortcutHookReturn = {
  pressedKeys: KeySet; // Which is Set<string>
  clearKeys: () => void;
};

export default function useKeyboardShortcut(
  targetKeys: string[],
  closeTargetKeys: string[],
  preventDefault: boolean
): KeyboardShortcutHookReturn {
  const [keys, setKeys] = useState<KeySet>(new Set());

  const clearKeys = useCallback(() => {
    setKeys((prevState) => new Set());
  }, [])

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (closeTargetKeys.includes(event.key)) {
        setKeys((prev) => new Set<string>([event.key]));
        return;
      }

      if (targetKeys.includes(event.key)) {
        if(preventDefault) {
          event.preventDefault();
        }
        setKeys((prevKeys) => new Set(prevKeys).add(event.key));
      }
    },
    [targetKeys, closeTargetKeys, preventDefault]
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

      if (closeTargetKeys.includes(event.key)) {
        setKeys((prev) => new Set());
      }
    },
    [targetKeys, closeTargetKeys]

  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [handleKeyDown, handleKeyUp]);

  return {pressedKeys: keys, clearKeys} ;
}
