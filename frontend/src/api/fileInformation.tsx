import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import type { FileInformation } from "../types/app";
import type { RAGDraggableElement } from "../components/DraggableElement";
import invariant from "tiny-invariant";

export default function useUploadAsEmbeddable() {
  const { sendRequest, ...misc } = useAxios<FileInformation, void>({
    url: import.meta.env.VITE_API + "embeddable/upload",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const uploadAsEmbeddable = useCallback(
    (draggableElement: RAGDraggableElement) => {
      invariant(
        draggableElement.id,
        "Id of draggable element musn't be null at this point"
      );
      return sendRequest({
        data: {
          id: draggableElement.id,
          name: draggableElement.name,
          path: draggableElement.path,
          len: draggableElement.len,
          ty: draggableElement.ty,
        },
      });
    },
    [sendRequest]
  );
  return { uploadAsEmbeddable, ...misc };
}

export function useGetFileInformations() {
  const { sendRequest } = useAxios<string[], FileInformation[]>({
    url: import.meta.env.VITE_API + "fileInformations/getByIds",
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const getFileInformations = useCallback(
    (ids: string[]) => {
      return sendRequest({ data: ids });
    },
    [sendRequest]
  );
  return { getFileInformations };
}
