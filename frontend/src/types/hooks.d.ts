interface AxiosParameter<RQD> {
  method?: "GET" | "POST" | "PUT" | "DELETE";
  url: string;
  data?: RQD;
  headers?: Record<string, string>;
}

interface AxiosFileParameter {
  url: string;
  method: "POST",
  len: number,
  file: File,
  headers? : Record<string, string>
}