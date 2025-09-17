import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import type { Draggable } from "../types/app";
import type { RAGDraggableElement } from "../components/DraggableElement";

export default function useGetAllDraggables() {
  const { sendRequest, ...misc } = useAxios<void, Draggable[]>({
    url: import.meta.env.VITE_API + "/draggable/getAll",
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const getAllDraggables = useCallback(() => {
    return sendRequest();
  }, [sendRequest]);
  return { getAllDraggables };
}

export function useSaveDraggable() {
  const { sendRequest, ...misc } = useAxios<Draggable, string>({
    url: import.meta.env.VITE_API + "/draggable/save",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  const saveDraggable = useCallback(
    (draggable: RAGDraggableElement) => {
      sendRequest({
        data: {
          id: draggable.id,
          position: draggable.position,
        },
      });
    },
    [sendRequest]
  );

  return { saveDraggable, ...misc };
}
