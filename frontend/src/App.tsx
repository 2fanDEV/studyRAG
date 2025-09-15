import { useState } from "react";
import { DndContext } from "@dnd-kit/core";
import FileSelectorButton from "./components/FileSelector";
import DraggableElement from "./components/DraggableElement";
import type { Draggable, FileInformation} from "./types/app";
import getAltOrCmdKey from "./util/os";

function App() {
  const [positions, setPositions] = useState<Draggable[]>([]);
  const [fileDetails, setFileDetails] = useState<FileInformation[]>([]);

  const handleUpload = (draggable: Draggable, fileInformation: FileInformation) => {
     setPositions((prev) => {
        let alreadyExistsIndex = prev.findIndex((p) => p.id === draggable.id);
        if (alreadyExistsIndex !== -1) {
          let i = 0;
          while (true) {
            let appendix = " (" + i + ")";
            if (prev.findIndex((p) => p.id === draggable.id + appendix) === -1) {
              draggable.id += appendix;
              break;
            }
            i++;
          }
        }
        return [...prev, draggable];
  })
    setFileDetails((prev) => [...prev, fileInformation]);
  }

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
            setPositions((prev) => {
              const id = active.id as string;
              return prev.map((p) =>
                p.id === id
                  ? {
                      ...p,
                      id: p.id,
                      position: {
                        x: p.position.x + delta.x,
                        y: p.position.y + delta.y,
                      },
                    }
                  : p
              );
            });
          }}
        >
          {Object.entries(positions).map(([, element]) => {
            let id = element.id;
            let fileInfo = fileDetails.find((f) => f.id === id);
            if (!fileInfo) return null;
            return (
              <DraggableElement
                {...element}
                {...fileInfo}
              />
            );
          })}
        </DndContext>
      </div>
    </div>
  );
}

export default App;
