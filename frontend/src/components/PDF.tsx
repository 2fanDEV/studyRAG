import { useDraggable, type UniqueIdentifier } from "@dnd-kit/core";
import type { Position } from "../types/app";

type PdfProps = {
  id: UniqueIdentifier;
  position: Position;
};

export default function PDF(props: PdfProps) {
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

  return (
    <button
      className="w-auto h-auto bg-[#1a536d] p-2 m-3 text-white font-semibold
	  border-transparent hover:border-teal-400 border-1 border-spacing-8
	  transition-colors ease-in-out duration-500
    hover:bg-[#1a5361] cursor-grab rounded-xl"
      ref={setNodeRef}
      style={style}
      {...listeners}
      {...attributes}
    >
      {props.id}
    </button>
  );
}
