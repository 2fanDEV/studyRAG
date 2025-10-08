import { LucidePlus } from "lucide-react";
import { Button } from "./ui/button";

export default function UploadButton() {
  return (
    <Button className="rounded-full w-8 h-8 bg-teal-700 border-1 border-teal-300 text-white">
      <LucidePlus> </LucidePlus>
    </Button>
  );
}
