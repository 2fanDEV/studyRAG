import { createContext, useContext } from "react";

export interface ShortCutContextType {
  isActivated: boolean;
  setIsActivated: (b: boolean) => void;
}

export const ShortcutContext = createContext<ShortCutContextType | undefined>(
  undefined
);

export const useQueryModalShortcut = () => useContext(ShortcutContext);
