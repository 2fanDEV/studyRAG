export interface Position  { x: number; y: number };
export interface Draggable  { 
	id: UniqueIdentifier,
	position: Position
};

export interface FileProps {
  name: string;
  ty: FileType;
  path: string,
  len: number;
}