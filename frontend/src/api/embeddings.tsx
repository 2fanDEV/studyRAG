import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import invariant from "tiny-invariant";

export default function useCreateEmbeddingsForId() {
  const { sendRequest } = useAxios<string, void>({
    url: import.meta.env.VITE_API + "/create_embedding",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const createEmbeddings = useCallback((id?: string) => {
        invariant(id, "Id must not be undefined at this point");

        return sendRequest({
            params: { id: id }
        })
  }, [sendRequest]);
  return { createEmbeddings };
}
