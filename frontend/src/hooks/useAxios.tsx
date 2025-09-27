import axios from "axios";
import { useCallback, useState } from "react";

export default function useAxios<RQD, RSD>(props: AxiosParameter<RQD>) {
  const [data, setData] = useState<RSD | undefined>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [uploadStarted, setUploadStarted] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [downloadStarted, setDownloadStarted] = useState(false);
  const sendRequest = useCallback(
    async (
      options?: Partial<AxiosParameter<RQD>>
    ): Promise<RSD | undefined> => {
      setLoading(true);
      setError(null);
      try {
        const defaultHeaders = {
          "Content-Type": "application/json",
          Accept: "application/json",
        };
        const response = await axios({
          url: options?.url || props.url,
          headers: options?.headers || defaultHeaders,
          method: options?.method || props.method,
          data: options?.data || props.data,
          params: options?.params || props.params,
          onUploadProgress: (event) => {
            setUploadStarted(true);
            if (event.total) {
              const percentage = Math.round((event.loaded * 100) / event.total);
              setUploadProgress(percentage);
            }
          },
          onDownloadProgress: (event) => {
            setDownloadStarted(true);
            if (event.total) {
              const percentage = Math.round((event.loaded * 100) / event.total);
              setDownloadProgress(percentage);
            }
          },
        });
        setData(response.data);
        return response.data;
      } catch (err: any) {
        setError(err.message || "Something went wrong!");
      } finally {
        setLoading(false);
      }
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

export function useAxiosChunked<RSD>(
  props: AxiosFileParameter,
  setUploadProgress: (progress: number) => void
) {
  const [data, setData] = useState<RSD | undefined>();
  const sendChunkedFileRequest = useCallback(
    async (
      id: string,
      options: Partial<AxiosFileParameter>
    ): Promise<RSD | undefined> => {
      if (!options.file) {
        return;
      }
      const chunkSize = Math.pow(1024, 2);
      console.log(chunkSize);
      const totalChunks = Math.ceil(options.file.size / chunkSize);

      for (let chunk = 0; chunk < totalChunks; chunk++) {
        const offset = chunk * chunkSize;
        const data = options.file.slice(offset, offset + chunkSize);
        const formData = new FormData();
        formData.append("id", id.toString());
        formData.append("chunk_data", data);
        formData.append("chunk_index", chunk.toString());
        formData.append("total_chunks", totalChunks.toString());
        try {
          const response = await axios({
            url: options.url || props.url,
            method: options.method || props.method,
            data: formData,
            headers: {
              "Content-Type": "multipart/form-data",
            },
          });
          if (chunk === totalChunks - 1) {
            setData(response.data);
            return response.data;
          }
        } catch (error: any) {
          return;
        }
      }
    },
    []
  );
  return { sendChunkedFileRequest };
}
