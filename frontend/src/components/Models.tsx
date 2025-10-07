import useGitHubRequests from "@/api/github";
import { useEffect, useState } from "react";
import { ItemMedia } from "./ui/item";
import { Spinner } from "./ui/spinner";
import type { Model } from "@/types/models.d";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";

export interface ModelProps {
  provider: string;
  models: Model[];
  selectedModel: (idx: string) => void;
}

export default function Models(props: ModelProps) {
  const { getModelsReq } = useGitHubRequests();
  return (
    <Select onValueChange={props.selectedModel}>
      <SelectTrigger className="w-auto text-xs">
        <SelectValue placeholder="Select Model" />
      </SelectTrigger>
      <SelectContent>
        {
            props.models.map((model: Model, idx: number) => {
               return <SelectItem key={idx} value="{model.id}">{model.name}</SelectItem> 
            })
        }
      </SelectContent>
    </Select>
  );
}
