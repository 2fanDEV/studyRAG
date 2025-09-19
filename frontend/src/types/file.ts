export const FileType = {
  PDF: "PDF",
} as const;

export type FileType = (typeof FileType)[keyof typeof FileType];