import LoginButton from "@/components/LoginButton";
import ModelProvider from "@/components/ModelProvider";
import ShortcutButton from "@/components/ShortCut";
import UploadButton from "@/components/UploadButton";
import { useModels } from "@/hooks/useModels";
import type { Model } from "@/types/models.d";

export function Menu() {
  const modelsCtx = useModels();

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
