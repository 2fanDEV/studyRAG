import { useEffect, useState } from "react";
import { DndContext } from "@dnd-kit/core";
import FileSelectorButton from "./components/FileSelector";
import DraggableElement, {
  type RAGDraggableElement,
} from "./components/DraggableElement";
import getControlOrCommandKey from "./util/os";
import useFetchAllDraggables, { useSaveDraggable } from "./api/draggable";
import type { FileInformation } from "./types/app";
import invariant from "tiny-invariant";
import useUploadFileInformation, {
  useFetchFileInformations,
  useSaveFileInformation,
} from "./api/fileInformation";
import useCreateEmbeddingsForId from "./api/embeddings";
import useKeyboardShortcut from "./hooks/useKeyboardShortcut";
import QueryModal from "./components/QueryModal";

function App() {
  const [draggableElements, setDraggableElements] = useState<
    RAGDraggableElement[]
  >([]);
  const { saveDraggable } = useSaveDraggable();
  const { fetchAllDraggables } = useFetchAllDraggables();
  const { saveFileInformation } = useSaveFileInformation();
  const { fetchFileInformations } = useFetchFileInformations();
  const { uploadFile } = useUploadFileInformation(() => {});
  const { createEmbeddings } = useCreateEmbeddingsForId();

  useEffect(() => {
    const fetchAndCombineData = async () => {
      const draggables = (await fetchAllDraggables()) || [];
      const ids = draggables
        .map((elem) => elem.id)
        .filter((elem) => elem !== undefined);
      let fileInfos = (await fetchFileInformations(ids)) || [];
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
      setDraggableElements(combinedElements);
    };
    fetchAndCombineData();
  }, [fetchAllDraggables, fetchFileInformations]);

  const handleUpload = async (item: RAGDraggableElement, file: File) => {
    try {
      const uuid = await saveDraggable(item);
      let updatedItem = {
        id: uuid,
        ...item,
      };
      await saveFileInformation(updatedItem);
      setDraggableElements((prev) => {
        let alreadyExistsIndex = prev.findIndex((p) => p.id === item.id);
        let newName = item.name;
        if (alreadyExistsIndex !== -1) {
          let i = 0;
          while (true) {
            let appendix = " (" + i + ")";
            if (prev.findIndex((p) => p.name === newName + appendix) === -1) {
              newName += appendix;
              break;
            }
            i++;
          }
        }
        return [...prev, { ...updatedItem, name: newName }];
      });
      await uploadFile(file, uuid);
      await createEmbeddings(uuid);
    } catch (error) {
      console.log(error);
    }
  };
  return (
    <div className="w-full h-full absolute bg-[#061319] -z-8">
      <QueryModal></QueryModal>
      <div
        className="absolute inset-0 
    [background-image:radial-gradient(white_1px,transparent_1px)] 
    [background-size:25px_25px]
    opacity-30 -z-1"
      ></div>
      <div className="flex gap-1 justify-end">
        <div className=" flex justify-center w-16 text-white self-center bg-transparent border-2 rounded-xl p-1 text-xs border-teal-400">
          <p> {getControlOrCommandKey().shortcut} + K </p>
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
          {Object.entries(draggableElements).map(([, element]) => {
            let id = element.id;
            return <DraggableElement {...element} />;
          })}
        </DndContext>
      </div>
    </div>
  );
}

export default App;
