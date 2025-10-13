import type { Model } from "@/types/models.d";

export interface Provider {
  id: string;
  name: string;
  models: Model[];
  models_url: string;
}

export const defaultProviders: Provider[] = [
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
    models_url: "http://localhost:11434/api/tags",
  },
];
