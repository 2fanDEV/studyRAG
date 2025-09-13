import { useState } from "react";
import { DndContext, type UniqueIdentifier } from "@dnd-kit/core";
import PDF from "./components/PDF";
import FileSelectorButton from "./components/FileSelector";
import type { DraggableElement } from "./types/app";

function App() {
  const [positions, setPositions] = useState<DraggableElement[]>([
    { id: "pdf_1", position: { x: 0, y: 0 } },
    { id: "pdf_2", position: { x: 100, y: 0 } },
  ]);
  setPositions;

  return (
    <div className="w-full h-full absolute bg-[#061319] -z-8">
      <div
        className="absolute inset-0 
    [background-image:radial-gradient(white_1px,transparent_1px)] 
    [background-size:25px_25px]
    opacity-30 -z-1"
      ></div>
      <div className="m-6 flex justify-end">
        <div>
          <FileSelectorButton name="Upload" draggableElements={setPositions} />
        </div>
      </div>

      <div className="w-full h-full text-white -z-1 bg-transparent">
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
            return <PDF id={element.id} position={element.position} />;
          })}
        </DndContext>
      </div>
    </div>
  );
}

export default App;
