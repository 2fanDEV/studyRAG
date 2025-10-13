import { LucideCommand, LucidePlus, LucideSend } from "lucide-react";
import { useCallback, useContext, useEffect } from "react";
import { Button } from "./ui/button";
import { useQueryContextShortcut } from "@/hooks/context/useShortcut";

export interface ShortcutProps {
  shortcutKey: string;
  shortcutCancelKey: string;
  metaKeyRequired: boolean;
}

const targetTo: Record<string, React.ReactElement> = {
  Meta: <LucideCommand/>,
  Enter: <LucideSend/>,
};

export default function ShortcutButton(props: ShortcutProps) {
  const ctx = useQueryContextShortcut();
  let shortcutKey = props.shortcutKey.toLowerCase();
  let cancelKey = props.shortcutCancelKey.toLowerCase();

  const handleKeyPress = useCallback(
    (event: KeyboardEvent) => {
      switch (event.key.toLowerCase()) {
        case shortcutKey:
          {
            if (event.metaKey) {
              event.preventDefault();
              ctx?.setIsActivated(true);
            }
          }
          break;
        case cancelKey:
          {
            ctx?.setIsActivated(false);
          }
          break;
      }
    },
    [ctx?.isActivated, ctx?.setIsActivated, shortcutKey, cancelKey]
  );

  useEffect(() => {
    document.addEventListener("keydown", handleKeyPress);
    return () => document.removeEventListener("keydown", handleKeyPress);
  }, [handleKeyPress]);

  return (
    <Button className="flex mt-2 mr-1 w-18 h-6 bg-black-700 text-white hover:bg-teal-800 border-teal-300 border-1 rounded-4xl gap-1.5 ">
      {props.metaKeyRequired ? targetTo["Meta"] : ""}
      {props.metaKeyRequired ? <LucidePlus/> : ""}
      {targetTo[props.shortcutKey] ? targetTo[props.shortcutKey]: <h1> {props.shortcutKey}</h1>}
    </Button>
  );
}
