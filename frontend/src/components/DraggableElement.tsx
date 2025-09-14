import { useDraggable, type UniqueIdentifier } from "@dnd-kit/core";
import PDF from "./PDF";
import type { FileProps, Position } from "../types/app";
import type { FileType } from "../types/file";

type DraggableElementProps = {
  id: UniqueIdentifier;
  position: Position;
  fileInformation: FileProps
};

export default function DraggableElement(props: DraggableElementProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: props.id,
  });

  


  const finalTransform = {
    x: props.position.x + (transform?.x ?? 0),
    y: props.position.y + (transform?.y ?? 0),
  };

  const style = {
    transform: `translate3d(${finalTransform.x}px, ${finalTransform.y}px, 0)`,
  };

  const element = (type: FileType) => { 
	switch(type){
	case "pdf": return <PDF {...props.fileInformation}></PDF>;
	default: return <div>Unsupported file type</div>
  }}
  return <div className="inline-block w-auto h-auto bg-yellow-600 cursor-grab text-xs"

  style={style} ref={setNodeRef} {...listeners} {...attributes}>
  {element(props.fileInformation.ty)}
  </div >;
}
