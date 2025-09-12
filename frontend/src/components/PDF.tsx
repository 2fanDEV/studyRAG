import { useDraggable, type UniqueIdentifier } from "@dnd-kit/core";

type Position = { x: number, y: number };
type PdfProps = {
	id: UniqueIdentifier,
	position: Position
}

export default function PDF(props: PdfProps) {
	const { attributes, listeners, setNodeRef, isDragging, transform } = useDraggable({
		id: props.id
	});

	const finalTransform = {
		x: props.position.x + (transform?.x ?? 0),
		y: props.position.y + (transform?.y ?? 0)
	}

	const style = {
		transform: `translate3d(${finalTransform.x}px, ${finalTransform.y}px, 0)`,
	}

	return (
		<button ref={setNodeRef} style={style} {...listeners} {...attributes}>
			{props.id}
		</button>
	);
}
