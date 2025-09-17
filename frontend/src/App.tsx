import { useEffect, useState } from "react";
import { DndContext } from "@dnd-kit/core";
import FileSelectorButton from "./components/FileSelector";
import DraggableElement, {
  type RAGDraggableElement,
} from "./components/DraggableElement";
import getAltOrCmdKey from "./util/os";
import useGetAllDraggables, { useSaveDraggable } from "./api/draggable";
import useUploadAsEmbeddable, {
  useGetFileInformations,
} from "./api/fileInformation";
import type { FileInformation } from "./types/app";
import invariant from "tiny-invariant";

function App() {
  const [draggableElements, setDraggableElements] = useState<
    RAGDraggableElement[]
  >([]);
  const { saveDraggable } = useSaveDraggable();
  const { getAllDraggables, ...misc } = useGetAllDraggables();
  const { uploadAsEmbeddable, ...uploadProps } = useUploadAsEmbeddable();
  const { getFileInformations } = useGetFileInformations();

  useEffect(() => {
    const fetchAndCombineData = async () => {
      const draggables = (await getAllDraggables()) || [];
      const ids = draggables
        .map((elem) => elem.id)
        .filter((elem) => elem !== undefined);

      let fileInfos = (await getFileInformations(ids)) || [];
      const fileMap = {} as Record<string, FileInformation>;
      if (fileInfos) {
        fileInfos.forEach((info) => {
          fileMap[info.id] = info;
        });
      }
      const combinedElements: RAGDraggableElement[] = draggables.map(
        (draggable) => {
          invariant(draggable.id, "Id cannot be null here!");
          const fileInfo = fileMap[draggable.id];
          return {
            ...draggable,
            ...fileInfo,
          };
        }
      );
      return combinedElements;
    };
    fetchAndCombineData().then((elements) => {
      setDraggableElements(elements);
    });
  }, [getAllDraggables]);

  const handleUpload = (item: RAGDraggableElement) => {
    setDraggableElements((prev) => {
      let alreadyExistsIndex = prev.findIndex((p) => p.id === item.id);
      if (alreadyExistsIndex !== -1) {
        let i = 0;
        while (true) {
          let appendix = " (" + i + ")";
          if (prev.findIndex((p) => p.id === item.id + appendix) === -1) {
            item.id += appendix;
            break;
          }
          i++;
        }
      }
      return [...prev, item];
    });
    saveDraggable(item);
    uploadAsEmbeddable(item);
  };

  return (
    <div className="w-full h-full absolute bg-[#061319] -z-8">
      <div
        className="absolute inset-0 
    [background-image:radial-gradient(white_1px,transparent_1px)] 
    [background-size:25px_25px]
    opacity-30 -z-1"
      ></div>
      <div className="flex gap-1 justify-end">
        <div className=" flex justify-center w-16 text-white self-center bg-transparent border-2 rounded-xl p-1 text-xs border-teal-400">
          <p> {getAltOrCmdKey()} + K </p>
        </div>
        <div className="m-6">
          <FileSelectorButton name="Upload" onUpload={handleUpload} />
        </div>
      </div>

      <div className="text-white -z-1 bg-transparent">
        <DndContext
          onDragEnd={({ delta, active }) => {
            setDraggableElements((prev) => {
              const id = active.id as string;
              let index = prev.findIndex((p) => p.id === id);
              let element = prev[index];
              const updated: RAGDraggableElement = {
                ...element,
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
          {Object.entries(draggableElements).map(([, element]) => {
            let id = element.id;
            return <DraggableElement {...element} />;
          })}
          <div> {uploadProps.uploadProgress}</div>
        </DndContext>
      </div>
    </div>
  );
}

export default App;
