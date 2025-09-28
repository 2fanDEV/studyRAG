import { Button } from "./ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "./ui/dialog";
import { DialogOverlay } from "@radix-ui/react-dialog";
import { Github } from "lucide-react";

export default function LoginButton(
) {



  const loginButton = (
    <Dialog>
      <DialogTrigger asChild>
        <Button
          variant="outline"
          
          className="
          header-element 
           rounded-2xl cursor-pointer"
        >
          Login
        </Button>
      </DialogTrigger>
      <DialogOverlay />
      <DialogContent className="bg-[#01131a]">
        <DialogHeader>
          <DialogTitle className="self-center">Login with</DialogTitle>
          <DialogDescription className="self-center text-xs">
            Select your Login Method
          </DialogDescription>
        </DialogHeader>
        <div className="flex flex-col-reverse justify-center items-center">
         <a href=""> <Button
            variant="outline"
            className="bg-[#032938] text-white cursor-pointer"
          >
            <div>
              <Github />
            </div>
            GitHub
          </Button>
          </a>
        </div>
        <DialogFooter>
          <DialogClose asChild></DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );

  return loginButton;
}
