import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { useEffect, useState } from "react";
import useProviderRequests from "@/api/provider";
import { defaultProviders, type Provider } from "@/types/provider";
import { useProvider } from "@/hooks/context/useProvider";
import type { Model, Models } from "@/types/models.d";

export interface ModelProviderProps {
  availableModels: (models: Model[]) => void;
}

export default function ModelProvider(providerProps: ModelProviderProps) {
  const { getModelsReq: getModels } = useProviderRequests(
    defaultProviders[0].models_url
  );
  const providerCtx = useProvider();
  const provider = providerCtx.provider;
  const [providers, setProviders] = useState<Provider[]>(defaultProviders);

  const selectProvider = (provider: string) => {
    const selectedProvider = providers.find((prov) => prov.name === provider);
    if (selectedProvider) {
      providerCtx.setProvider(selectedProvider);
    }
  };

  const isOllama= (models: Models) => {
      return "models" in models;
  }

  useEffect(() => {
    const getModelsOfProvider = async (provider: Provider) => {
      let provider_models = (await getModels(provider.models_url)) || [];
    
        providerProps.availableModels(isOllama(provider_models) ? provider_models.models : provider_models  );
    };
    getModelsOfProvider(provider);
  }, [provider]);

  let providerSelection = (
    <Select onValueChange={selectProvider} defaultValue={provider.name}>
      <SelectTrigger className="w-auto mt-5 text-md">
        <SelectValue defaultChecked={true} />
      </SelectTrigger>
      <SelectContent>
        {providers.map((prov, idx: number) => {
          return (
            <SelectItem key={prov.name} value={prov.name}>
              {prov.name}
            </SelectItem>
          );
        })}
      </SelectContent>
    </Select>
  );

  return (
    <div className=" bg-transparent underline border-none text-white">
      {providerSelection}
    </div>
  );
}
