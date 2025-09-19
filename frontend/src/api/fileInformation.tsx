import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import type { FileInformation } from "../types/app";
import type { RAGDraggableElement } from "../components/DraggableElement";
import invariant from "tiny-invariant";

export default function useUploadFileInformation() {
  const { sendRequest, ...misc } = useAxios<FileInformation, void>({
    url: import.meta.env.VITE_API + "/embeddable/upload",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const uploadFileInformation = useCallback(
    (draggableElement: RAGDraggableElement) => {
      invariant(
        draggableElement.id,
        "Id of draggable element musn't be null at this point"
      );
      return sendRequest({
        data: { 
          id: draggableElement.id,
          name: draggableElement.name,
          len: draggableElement.len,
          ty: draggableElement.ty,
        },
      });
    },
    [sendRequest]
  );
  return { uploadFileInformation, ...misc };
}

export function useFetchFileInformations() {
  const { sendRequest, ...misc }= useAxios<string[], FileInformation[]>({
    url: import.meta.env.VITE_API + "/fileInformations/getByIds",
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const fetchFileInformations = useCallback(
    (ids: string[]) => {
      return sendRequest({ data: ids });
    },
    [sendRequest]
  );
  return { fetchFileInformations, ...misc };
}
