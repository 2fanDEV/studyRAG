export const FileType = {
  PDF: "pdf",
} as const;

export type FileType = (typeof FileType)[keyof typeof FileType];