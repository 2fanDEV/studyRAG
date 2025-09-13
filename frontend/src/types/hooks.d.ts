type AxiosParameter = {
    method?: "GET" | "POST" | "PUT" | "DELETE";
    url: string;
    data?: any;
    headers?: Record<string, string>;
}