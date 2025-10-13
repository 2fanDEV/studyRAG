import useAxios from "@/hooks/useAxios";
import type { Models } from "@/types/models.d";

export default function useProviderRequests(url: string) {
  const { sendRequest: getModelsRequest } = useAxios<string, Models>({
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
