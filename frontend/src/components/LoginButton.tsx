import { useEffect, useState } from "react";
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
import useTokenExchange from "@/api/auth";

export default function LoginButton() {
  const [loggedIn, setLoggedIn] = useState(false);
  const { exchangeToken } = useTokenExchange();

  useEffect(() => {
    const exchange = async () => {
      const searchParams = new URLSearchParams(window.location.search);
      const code = searchParams.get("code");
      if (code) {
        let token = await exchangeToken(code);
        if (token) {
          window.cookieStore.set("ght", token.access_token);
          setLoggedIn(true);
        }
      }
      window.history.replaceState("", document.title, window.location.origin);
    };
    exchange();
  }, []);

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
          <a
            href={
              import.meta.env.VITE_GITHUB_AUTH_BASE +
              import.meta.env.VITE_GITHUB_CLIENT_ID
            }
          >
            <Button
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
