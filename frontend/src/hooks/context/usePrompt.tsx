import { useContext } from "react";


export interface PromptContextType {
    prompt: string
    setPrompt: (prompt: string) => void;
}

import { createContext } from "react";

export const PromptContext = createContext<PromptContextType | undefined>(undefined);

export const useSystemPrompt = () =>  useContext(PromptContext);

