import { useState } from "react";
import type { Model } from "./types/models.d";
import { DraggableCanvas } from "./composition/DraggableCanvas";
import { Menu } from "./composition/Menu";
import { ShortcutContext } from "./hooks/useShortcut";
import QueryInput from "./components/QueryInput";
import { ListModelsContext } from "./hooks/useModels";
import { ModelContext } from "./hooks/useSelectedModel";
import { LucideCommand } from "lucide-react";

function App() {
  const [model, setModel] = useState<Model | undefined>(undefined);
  const [models, setModels] = useState<Model[]>([]);
  const [prompt, setPrompt] = useState<string>("");
  const [isQueryActivated, setIsQueryActivated] = useState(false);

  let queryInput = isQueryActivated ? (
    <QueryInput
      prompt={prompt}
      setPrompt={setPrompt}
      submit={(b) => {
        if (b) {
          console.log("SUBMITTED", prompt);
        }
      }}
    />
  ) : (
    <div></div>
  );
  return (
    <div>
      <ListModelsContext value={{ models, setModels }}>
        <ModelContext value={{ model, setModel }}>
          <ShortcutContext
            value={{
              isActivated: isQueryActivated,
              setIsActivated: setIsQueryActivated,
            }}
          >
            <Menu />
            {queryInput}
            <DraggableCanvas />
          </ShortcutContext>
        </ModelContext>
      </ListModelsContext>
    </div>
  );
}

export default App;
