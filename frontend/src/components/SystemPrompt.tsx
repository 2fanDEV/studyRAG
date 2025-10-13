import { useSystemPrompt } from "@/hooks/context/usePrompt";
import AutoExpandingTextArea from "./AutoExpandingTextArea";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "./ui/dialog";

export default function SystemPromptModal() {
  const ctx = useSystemPrompt();

  return (
    <Dialog open={true}>
      <DialogContent className="text-xs">
        <DialogHeader>
          <DialogTitle className="self-center">
            Customize System Prompt
          </DialogTitle>
        </DialogHeader>
          <AutoExpandingTextArea
            text={ctx ? ctx.prompt : ""}
            placeholder="System prompt"
            inputCallback={function (input: string): void {
              throw new Error("Function not implemented.");
            }}
          ></AutoExpandingTextArea>
      </DialogContent>
    </Dialog>
  );
}
