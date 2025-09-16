import axios from "axios";
import { useCallback, useState } from "react";

export default function useAxios<T>(props: AxiosParameter){
  let [data, setData] = useState<T | null >(null);
  let [loading, setLoading] = useState(false);
  let [error, setError] = useState<string | null>(null);
  let [uploadProgress, setUploadProgress] = useState(0);
  let [uploadStarted, setUploadStarted] = useState(false);
  let [downloadProgress, setDownloadProgress] = useState(0);
  let [downloadStarted, setDownloadStarted] = useState(false);
  const sendRequest= useCallback(
    async (options?: Partial<AxiosParameter>): Promise<T | null> => {
      setLoading(true);
      setError(null);
      try {
        const response = await axios({
          url: options?.url || props.url,
          headers: options?.headers || props.headers,
          method: options?.method || props.method,
          data: options?.data || props.data,
          onUploadProgress: (event) => {
            setUploadStarted(true);
            if (event.total) {
              const percentage = Math.round((event.loaded * 100) / event.total);
              console.log(percentage);
              setUploadProgress(percentage);
            }
          },
          onDownloadProgress: (event) => {
            setDownloadStarted(true);
            if (event.total) {
              const percentage = Math.round((event.loaded * 100) / event.total);
              console.log(percentage);
              setDownloadProgress(
                percentage
              );
            }
          },
        });
        setData(response.data);
        return data;
      } catch (err: any) {
        setError(err.message || "Something went wrong!");
      } finally {
        setLoading(false);
      }
      return null;
    },
    []
  );
  return {
    data,
    loading,
    error,
    uploadProgress,
    uploadStarted,
    downloadProgress,
    downloadStarted,
    sendRequest,
  };
}
