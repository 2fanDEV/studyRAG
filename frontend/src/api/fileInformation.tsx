import { useCallback } from "react";
import useAxios, { useAxiosChunked } from "../hooks/useAxios";
import type { FileInformation } from "../types/app";
import type { RAGDraggableElement } from "../components/DraggableElement";
import invariant from "tiny-invariant";

export default function useUploadFileInformation(
  setUploadProgress: () => void
) {
  const { sendChunkedFileRequest, ...misc } = useAxiosChunked<void>(
    {
      url: import.meta.env.VITE_API + "/file_information/upload",
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      len: 0,
    },
    setUploadProgress
  );

  const uploadFile= useCallback(
    (file: File, id?: string) => {
      invariant(id, "At this point, id must not be undefined")
      return sendChunkedFileRequest(id, { file: file });
    },
    [sendChunkedFileRequest]
  );
  return { uploadFile, ...misc };
}
export function useSaveFileInformation() {
  const { sendRequest, ...misc } = useAxios<FileInformation, void>({
    url: import.meta.env.VITE_API + "/file_information/save",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const saveFileInformation = useCallback(
    (draggableElement: RAGDraggableElement) => {
      invariant(draggableElement.id, "At this point the ID must've been set!")
      return sendRequest({
        data: {
          id: draggableElement.id,
          len: draggableElement.len,
          name: draggableElement.name,
          ty: draggableElement.ty
        },
      });
    },
    [sendRequest]
  );
  return { saveFileInformation, ...misc };
}

export function useFetchFileInformations() {
  const { sendRequest, ...misc } = useAxios<string[], FileInformation[]>({
    url: import.meta.env.VITE_API + "/file_information/get_by_ids",
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const fetchFileInformations = useCallback(
    (ids: string[]) => {
      return sendRequest({ params: { ids: ids } });
    },
    [sendRequest]
  );
  return { fetchFileInformations, ...misc };
}
