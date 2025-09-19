import axios from "axios";
import { useCallback, useState } from "react";

export default function useAxios<RQD, RSD>(props: AxiosParameter<RQD>){
  const [data, setData] = useState<RSD | undefined>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [uploadStarted, setUploadStarted] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [downloadStarted, setDownloadStarted] = useState(false);
  const sendRequest= useCallback(
    async (options?: Partial<AxiosParameter<RQD>>): Promise<RSD| undefined> => {
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


  export function useAxiosChunked<RSD>(props: AxiosFileParameter, setUploadProgress: (progress: number) => void) {
  const [data, setData] = useState<RSD | undefined>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [uploadProgress, onUploadProgress] = useState(0); 

  const sendChunkedFileRequest = useCallback(
    async (options: Partial<AxiosFileParameter>): Promise<RSD | undefined> =>
  {
        setLoading(true);
        setError(null);
        if(!options.file) {
          return ;
        }
        const chunkSize = Math.pow(1024, 2);
        const totalChunks = Math.ceil(options.file.size/chunkSize);

        for(let chunk = 0; chunk < totalChunks; chunk++) {
            const offset = chunk * chunkSize;
            const data = options.file.slice(offset, offset + chunkSize);
            const formData= new FormData();
            formData.append("chunk_data", data);
            formData.append("chunk_index", chunk.toString());
            formData.append("total_chunks", totalChunks.toString());
            formData.append("file_name", File.name);
            try {
            const response=  await axios({
              url: options.url || props.url,
              data: formData,
              headers: {
                  'Content-Type': 'multipart/form-data',
              },
              onUploadProgress: (event) => {
                if(event.progress) {
                  setUploadProgress(event.progress);
                }
              }
            });
            setData(response.data);
            return response.data;
          } catch(error: any) {
            setError(error);
            setLoading(false);
            return;
          }
        }
  }, [])
    setLoading(false);
    return { sendChunkedFileRequest, loading, uploadProgress, error};
}
