import LoginButton from "@/components/LoginButton";
import ModelProvider from "@/components/ModelProvider";
import ShortcutButton from "@/components/ShortCut";
import UploadButton from "@/components/UploadButton";
import { useModels } from "@/hooks/context/useModels";
import { ShortcutContext } from "@/hooks/context/useShortcut";
import type { Model } from "@/types/models.d";
import { useEffect, useState } from "react";

export interface MenuProps {
  setSystemPromptDialog: (b: boolean) => void;
}

export function Menu(props: MenuProps) {
  const modelsCtx = useModels();
  const [isActive, setIsActivate] = useState(false);

  useEffect(() => {
       props.setSystemPromptDialog(isActive);
  }, [isActive])

  return (
    <div className="grid w-full grid-cols-2 absolute">
      <div className="">
        <ModelProvider
          availableModels={(model: Model[]) => {
            console.log(model);
            modelsCtx?.setModels(model);
          }}
        />
      </div>
      <div className="w-full p-5 gap-5 flex justify-end z-10">
        <ShortcutContext.Provider
          value={{ isActivated: isActive, setIsActivated: setIsActivate }}
        >
          <ShortcutButton
            shortcutKey="S"
            shortcutCancelKey="Escape"
            metaKeyRequired={true}
          />
        </ShortcutContext.Provider>
        <ShortcutButton
          shortcutKey={"K"}
          shortcutCancelKey={"Escape"}
          metaKeyRequired={true}
        />
        <div className="mt-1">
          <UploadButton />
        </div>
        <div className="mt-1.5">
          <LoginButton />
        </div>
      </div>
    </div>
  );
}
