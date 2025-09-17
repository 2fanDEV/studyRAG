import { useDraggable } from "@dnd-kit/core";
import PDF from "./PDF";
import type { Draggable, FileInformation } from "../types/app";
import type { FileType } from "../types/file";

export type RAGDraggableElement = Omit<Draggable & FileInformation, 'id'> & { 
  id?: string | undefined;
};

export default function DraggableElement(props: RAGDraggableElement) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: props.id || "0000",
  });

  const finalTransform = {
    x: props.position.x + (transform?.x ?? 0),
    y: props.position.y + (transform?.y ?? 0),
  };

  const style = {
    transform: `translate3d(${finalTransform.x}px, ${finalTransform.y}px, 0)`,
  };

  const element = (type: FileType) => {
    switch (type) {
      case "pdf":
        return <PDF {...props}></PDF>;
      default:
        return <div>Unsupported file type</div>;
    }
  };
  return (
    <div
      className="inline-block w-auto h-auto bg-yellow-600 cursor-grab text-xs"
      style={style}
      ref={setNodeRef}
      {...listeners}
      {...attributes}
    >
      {element(props.ty)}
    </div>
  );
}
