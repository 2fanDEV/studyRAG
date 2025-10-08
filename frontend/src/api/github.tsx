import useAxios from "@/hooks/useAxios";
import type { Model } from "@/types/models.d";

export default function useProviderRequests(url: string) {
  const { sendRequest: getModelsRequest } = useAxios<string, Model[]>({
    url: "url",
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      //       Accept: "application/vnd.github+json",
      Accept: "application/json",
    },
  });

  const getModelsReq = (url: string) => {
    return getModelsRequest({ url });
  };

  return { getModelsReq };
}
