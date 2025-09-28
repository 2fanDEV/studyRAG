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
import QueryModal from "./components/QueryModal";
import LoginButton from "./components/LoginButton";


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
    <div className="w-full h-screen bg-[#01131a]
       [background-image:radial-gradient(white_0.75px,transparent_0.5px)] 
    [background-size:2.5rem_2.5rem] opacity-100 
    ">
      <QueryModal></QueryModal>
      <div className="flex gap-0.5 justify-end">
        <div className="flex header-element items-center justify-center w-12 text-white self-center bg-transparent border-1 rounded-xl text-xs border-teal-400">
          <p> {getControlOrCommandKey().icon()} </p>K
        </div>
        <div className="m-6">
          <FileSelectorButton name="Upload" onUpload={handleUpload} />
        </div>
        <div className="mt-6 mr-4">
          <LoginButton></LoginButton>
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
          {Object.entries(draggableElements).map(([_, element]) => {
            return <DraggableElement key={element.id} {...element} />;
          })}
        </DndContext>
      </div>
    </div>
  );
}

export default App;
