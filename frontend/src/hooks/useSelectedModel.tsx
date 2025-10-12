import type { Model } from "@/types/models.d";
import { createContext, useContext } from "react";

export interface ModelContextType {
  model: Model | undefined;
  setModel: (model: Model) => void;
}

export const ModelContext = createContext<ModelContextType | undefined>(
  undefined
);

export const useSelectedModel = () => useContext(ModelContext);
