export type Model = GitHubModelSchema | OllamaModelDetails

export interface GitHubModelSchema {
  id: string;
  name: string;
  publisher: string;
  registry: string;
  summary: string;
  html_url: string;
  version: string;
  capabilities: string[];
  limits: Limits;
  rate_limit_tier: string;
  supported_input_modalities: string[];
  supported_output_modalities: string[];
  tags: string[];
}

interface Limits {
  max_input_tokens: number;
  max_output_tokens: number;
}

export interface OllamaModel {
  name: string;
  model: string;
  modified_at: string;
  size: number;
  digest: string;
  details: OllamaModelDetails;
}

interface OllamaModelDetails {
  parent_model: string;
  format: string;
  family: string;
  families: string[];
  parameter_size: string;
  quantization_level: string;
}


