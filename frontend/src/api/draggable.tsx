import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import type { Draggable } from "../types/app";
import type { RAGDraggableElement } from "../components/DraggableElement";

export default function useFetchAllDraggables() {
  const { sendRequest, ...misc } = useAxios<void, Draggable[]>({
    url: import.meta.env.VITE_API + "/draggable/all/0",
    method: "GET",
  });

  const fetchAllDraggables = useCallback(() => {
    return sendRequest();
  }, [sendRequest]);
  return { fetchAllDraggables, ...misc };
}

export function useSaveDraggable() {
  const { sendRequest, data, ...misc } = useAxios<Draggable, string>({
    url: import.meta.env.VITE_API + "/draggable/save",
    method: "POST",
 
  });

  const saveDraggable = useCallback(
    (draggable: RAGDraggableElement) => {
      return sendRequest({
        data: {
          id: draggable.id,
          position: draggable.position,
        },
      });
    },
    [sendRequest]
  );

  return { saveDraggable, data, ...misc };
}
