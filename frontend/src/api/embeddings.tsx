import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import invariant from "tiny-invariant";

export interface QueryRequest {
  text: string;
}

export default function useCreateEmbeddingsForId() {
  const { sendRequest } = useAxios<string, void>({
    url: import.meta.env.VITE_API + "/create_embedding",
    method: "POST",
  });

  const createEmbeddings = useCallback(
    (id?: string) => {
      invariant(id, "Id must not be undefined at this point");
      return sendRequest({
        params: { id: id },
      });
    },
    [sendRequest]
  );
  return { createEmbeddings };
}

export function useSendQuery() {
  const { sendRequest } = useAxios<QueryRequest, any>({
    url: import.meta.env.VITE_API + "/send_query",
    method: "POST",
  });

  const sendQuery = useCallback(
    (query: QueryRequest) => {
      invariant(query, "text must be defined");
      return sendRequest({
        data: query,
      });
    },
    [sendRequest]
  );

  return { sendQuery };
}
