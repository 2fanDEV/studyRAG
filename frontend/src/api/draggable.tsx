import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import type { Draggable } from "../types/app";
import type { RAGDraggableElement } from "../components/DraggableElement";

export function useDraggable() {
  const { sendRequest: saveDraggableRequest, data, ...saveDraggablesMisc } = useAxios<Draggable, string>({
    url: import.meta.env.VITE_API + "/draggable/save",
    method: "POST",
 
  });
  const { sendRequest: fetchAllDraggablesRequest, ...allDraggablesMisc } = useAxios<void, Draggable[]>({
    url: import.meta.env.VITE_API + "/draggable/all/0",
    method: "GET",
  });

  const fetchAllDraggables = useCallback(() => {
    return fetchAllDraggablesRequest();
  }, [fetchAllDraggablesRequest]);

  const saveDraggable = useCallback(
    (draggable: RAGDraggableElement) => {
      return saveDraggableRequest({
        data: {
          id: draggable.id,
          position: draggable.position,
        },
      });
    },
    [saveDraggableRequest]
  );

  return { saveDraggable, fetchAllDraggables};
}
