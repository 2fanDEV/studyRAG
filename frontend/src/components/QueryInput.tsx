import { useEffect, useState } from "react";
import AutoExpandingTextArea from "./AutoExpandingTextArea";
import { ShortcutContext, useQueryContextShortcut } from "@/hooks/context/useShortcut";
import ShortcutButton from "./ShortCut";
import { useModels } from "@/hooks/context/useModels";
import { Spinner } from "./ui/spinner";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { useSelectedModel } from "@/hooks/context/useSelectedModel";
import type { Model } from "@/types/models.d";

export interface QueryProps {
  prompt: string;
  setPrompt: (prompt: string) => void;
  submit: (b: boolean) => void;
}

export default function QueryInput(props: QueryProps) {
  const firstIndexForSelect = "0";
  const queryContext = useQueryContextShortcut();
  const modelsCtx = useModels();
  const modelCtx = useSelectedModel();
  const [submitted, setSubmitted] = useState(false);

  useEffect(() => {
    if (submitted) {
      if (queryContext) {
        queryContext.setIsActivated(false);
      }
      props.submit(true);
      props.setPrompt("");
    }
  }, [submitted]);
  
  const inputCallback = async (prompt: string) => {
    props.setPrompt(prompt);
  };

  const valueChangeHandle = (value: string) => {
    let index = Number.parseInt(value);
    if (modelsCtx && modelCtx) {
      modelCtx.setModel(modelsCtx.models[index]);
    }
  };

  let loadedModels = modelsCtx ? (
    <Select
      onValueChange={valueChangeHandle}
      defaultValue={firstIndexForSelect}
    >
      <SelectTrigger className="w-auto text-md">
        <SelectValue defaultChecked={true} />
      </SelectTrigger>
      <SelectContent>
        {modelsCtx.models.map((model, idx) => 
        {
          return (
            <SelectItem value={idx.toString()} key={model.name}>
              {model.name}
            </SelectItem>
          );
        })}
      </SelectContent>
    </Select>
  ) : (
    <Spinner className="mt-3.5 ml-15"></Spinner>
  );

  let dialog = (
    <ShortcutContext.Provider
      value={{ isActivated: submitted, setIsActivated: setSubmitted }}
    >
      <div className="w-full h-full absolute flex justify-center ">
        <div className="flex flex-col justify-center text-white bg-transparent w-md h-full">
          <div className="self-center">
            <div
              className="
              text-white
              rounded-2xl 
              p-4
              w-xl
              drop-shadow-white
              drop-shadow-xs
              mr-5
              animate-fadeIn0_20
              border-white bg-[linear-gradient(to_top,#00808077,transparent)]
              backdrop-blur-[1px]"
            >
              <AutoExpandingTextArea
                text={props.prompt}
                placeholder="Ask about something"
                inputCallback={inputCallback}
              ></AutoExpandingTextArea>
            </div>
          </div>
          <div className="grid grid-cols-2 w-full opacity-0 animate-fadeIn0_20 mt-3">
            <div className="justify-self-start -ml-4"> {loadedModels} </div>
            <div className="justify-self-end mr-2">
              <ShortcutButton
                shortcutKey={"Enter"}
                shortcutCancelKey={""}
                metaKeyRequired={true}
              />
            </div>
          </div>
        </div>
      </div>
    </ShortcutContext.Provider>
  );

  return dialog;
}
