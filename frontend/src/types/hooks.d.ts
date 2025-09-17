interface AxiosParameter<RQD> {
  method?: "GET" | "POST" | "PUT" | "DELETE";
  url: string;
  data?: T;
  headers?: Record<string, string>;
}
