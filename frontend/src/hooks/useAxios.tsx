import axios from "axios";
import { useCallback, useState } from "react";

export default function useAxios(props: AxiosParameter) {
  let [data, setData] = useState(null);
  let [loading, setLoading] = useState(true);
  let [error, setError] = useState(null);
  let [uploadProgress, setUploadProgress] = useState(0);
  let [uploadStarted, setUploadStarted] = useState(false);
  let [downloadProgress, setDownloadProgress] = useState(0);
  let [downloadStarted, setDownloadStarted] = useState(false);

  const sendRequest = useCallback(
    async (options?: Partial<AxiosParameter>) => {
      setLoading(true);
      setError(null);
      try {
        const response = await axios({
          url: options?.url,
          headers: options?.headers || props.headers,
          method: options?.method || "GET",
          data: options?.data || props.data,
          onUploadProgress: (event) => {
            setUploadStarted(true);
            if (event.total) {
              setUploadProgress(Math.round((event.loaded * 100) / event.total));
            }
          },
          onDownloadProgress: (event) => {
            setDownloadStarted(true);
            if (event.total) {
              setDownloadProgress(
                Math.round((event.loaded * 100) / event.total)
              );
            }
          },
        });
        setData(response.data);
      } catch (err: any) {
        setError(err.message || "Something went wrong!");
      } finally {
        setLoading(false);
      }
    },
    [props.url, props.headers, props.method, props.data]
  );

  return null;
}
