import useAxios from "@/hooks/useAxios";
import type { Model } from "@/types/models.d";

export default function useGitHubRequests() {
  const { sendRequest: getModelsRequest } = useAxios<string, Model[]>({
    url: "/api/catalog/models",
    method: "GET",
    headers: {
        "Content-Type": "application/json",
        Accept: "application/vnd.github+json",
    }
  });

  const getModelsReq = () => {
    return getModelsRequest();
  };

  return { getModelsReq };
}
