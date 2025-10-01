import useAxios from "@/hooks/useAxios";
import { useCallback } from "react";


export interface TokenResponse {
   access_token: string,
   token_type: string,
   scope: string
}

export default function useTokenExchange() {
  const { sendRequest } = useAxios<string, TokenResponse>({
    url: import.meta.env.VITE_API + "/auth",
    method: "POST",
  });

  const exchangeToken = useCallback(
    (code: string) => {
      return sendRequest({ params: { code: code } });
    },
    [sendRequest]
  );

  return { exchangeToken };
}
