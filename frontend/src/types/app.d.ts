export interface Position {
  x: number;
  y: number;
}

export interface Draggable {
  id?: string;
  position: Position;
}

export interface FileInformation {
  id: string;
  name: string;
  ty: FileType;
  len: number;
}
