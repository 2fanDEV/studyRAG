import { useState } from "react";
import type { Model } from "./types/models.d";
import { DraggableCanvas } from "./composition/DraggableCanvas";
import { Menu } from "./composition/Menu";
import { ShortcutContext } from "./hooks/context/useShortcut";
import QueryInput from "./components/QueryInput";
import { ListModelsContext } from "./hooks/context/useModels";
import { ModelContext } from "./hooks/context/useSelectedModel";
import SystemPromptModal from "./components/SystemPrompt";
import { PromptContext } from "./hooks/context/usePrompt";
import { DEFAULT_SYSTEM_PROMPT } from "./constants";
import { ProviderContext } from "./hooks/context/useProvider";
import { defaultProviders, type Provider } from "./types/provider";

function App() {
  const [model, setModel] = useState<Model | undefined>(undefined);
  const [models, setModels] = useState<Model[]>([]);
  const [prompt, setPrompt] = useState<string>("");
  const [systemPrompt, setSystemPrompt] = useState<string>(
    DEFAULT_SYSTEM_PROMPT
  );
  const [provider, setProvider] = useState<Provider>(defaultProviders[0]);
  const [isQueryActivated, setIsQueryActivated] = useState(false);
  const [showSystemPrompt, setShowSystemPrompt] = useState(false);

  const handlePrompt = (prompt: string) => {
    setPrompt(prompt);
  };

  let systemPromptCustomizer = showSystemPrompt ? <SystemPromptModal /> : "";

  let queryInput = isQueryActivated ? (
    <QueryInput
      prompt={prompt}
      setPrompt={handlePrompt}
      submit={(b) => {
        if (b) {
          console.log(prompt);
        }
      }}
    />
  ) : (
    <div></div>
  );
  return (
    <div>
      <ProviderContext value={{ provider, setProvider }}>
        <PromptContext
          value={{ prompt: systemPrompt, setPrompt: setSystemPrompt }}
        >
          <ListModelsContext value={{ models, setModels }}>
            <ModelContext value={{ model, setModel }}>
              <ShortcutContext
                value={{
                  isActivated: isQueryActivated,
                  setIsActivated: setIsQueryActivated,
                }}
              >
                <Menu setSystemPromptDialog={setShowSystemPrompt} />
                {systemPromptCustomizer}
                {queryInput}
                <DraggableCanvas />
              </ShortcutContext>
            </ModelContext>
          </ListModelsContext>
        </PromptContext>
      </ProviderContext>
    </div>
  );
}

export default App;
