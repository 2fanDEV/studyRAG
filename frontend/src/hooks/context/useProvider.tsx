import { defaultProviders, type Provider } from "@/types/provider";
import { createContext, useContext } from "react";

export interface ProviderContextType {
  provider: Provider;
  setProvider: (provider: Provider) => void;
}

export const ProviderContext = createContext<ProviderContextType>({
  provider: defaultProviders[0],
  setProvider: (provider: Provider) => {},
});

export const useProvider = () => useContext(ProviderContext);
