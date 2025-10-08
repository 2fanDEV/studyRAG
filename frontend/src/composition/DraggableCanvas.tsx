import { useDraggable } from "@/api/draggable";
import useFileUtilities, { useFileRetrieval } from "@/api/fileInformation";
import type { RAGDraggableElement } from "@/components/DraggableElement";
import DraggableElement from "@/components/DraggableElement";
import type { FileInformation } from "@/types/app";
import { DndContext } from "@dnd-kit/core";
import { useEffect, useState } from "react";
import invariant from "tiny-invariant";

export function DraggableCanvas() {
  const [draggableElements, setDraggableElements] = useState<
    RAGDraggableElement[]
  >([]);
  const { saveDraggable, fetchAllDraggables } = useDraggable();
  const { fetchFileInformations } = useFileRetrieval();

  useEffect(() => {
    const retrieveDraggableElements = async () => {
      const draggables = await fetchAllDraggables();

      invariant(draggables, "");

      const ids = draggables
        .map((d) => d.id)
        .filter((elem) => elem !== undefined);
      let fileInfos = await fetchFileInformations(ids);

      invariant(fileInfos, "FileInfos has to be initialized at this point");

      const fileMap = {} as Record<string, FileInformation>;
      fileInfos.forEach((info) => {
        fileMap[info.id] = info;
      });

      const combinedElements: RAGDraggableElement[] = draggables.map((d) => {
        invariant(d.id, "Id cannot be null here!");
        const fileInfo = fileMap[d.id];
        return {
          ...d,
          ...fileInfo,
        };
      });
      setDraggableElements(combinedElements);
    };
    retrieveDraggableElements();
  }, [fetchAllDraggables]);

  return (
    <div
      className="w-full h-screen bg-[#01131a]
       [background-image:radial-gradient(white_0.75px,transparent_0.5px)] 
    [background-size:2.5rem_2.5rem] opacity-100 z-0
    "
    >
      <div className="text-white">
        <DndContext
          onDragEnd={({ delta, active }) => {
            setDraggableElements((prev) => {
              const id = active.id as string;
              let index = prev.findIndex((p) => p.id === id);
              let element = prev[index];
              const updated: RAGDraggableElement = {
                ...element,
                id: id,
                position: {
                  x: element.position.x + delta.x,
                  y: element.position.y + delta.y,
                },
              };
              saveDraggable(updated);
              return prev.map((p) => (p.id === id ? updated : p));
            });
          }}
        >
          {Object.entries(draggableElements).map(([_, element]) => {
            return <DraggableElement key={element.id} {...element} />;
          })}
        </DndContext>
      </div>
    </div>
  );
}
