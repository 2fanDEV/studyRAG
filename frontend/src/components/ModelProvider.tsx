import useGitHubRequests from "@/api/github";
import type { Model } from "@/types/models.d";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { useEffect, useState } from "react";
import useProviderRequests from "@/api/github";

export interface Provider {
  id: string;
  name: string;
  models: Model[];
  models_url: string;
}

export interface ModelProviderProps {
  availableModels: (models: Model[]) => void;
}

const defaultProviders: Provider[] = [
  {
    id: "GITHUB",
    name: "GitHub",
    models: [],
    models_url: "/api/catalog/models",
  },
  {
    id: "OLLAMA",
    name: "Ollama",
    models: [],
    models_url: "",
  },
];

export default function ModelProvider(providerProps: ModelProviderProps) {
  const { getModelsReq: getModels } = useProviderRequests(
    defaultProviders[0].models_url
  );
  const [provider, setProvider] = useState<Provider>(defaultProviders[0]);
  const [providers, setProviders] = useState<Provider[]>(defaultProviders);

  const newProvider = (provider: Provider) => {
    setProviders((prevState) => {
      return [...prevState, provider];
    });
  };

  const selectProvider = (provider: string) => {
    const selectedProvider = providers.find((prov) => prov.name === provider);
    if (selectedProvider) {
      setProvider(selectedProvider);
    }
  };

  useEffect(() => {
    const getModelsOfProvider = async (provider: Provider) => {
      let provider_models = (await getModels(provider.models_url)) || [];
      providerProps.availableModels(provider_models);
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
