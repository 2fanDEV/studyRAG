export type OS = "Windows" | "MacOS" | "Linux" | "Unknown";
import { ChevronUp, Command } from "lucide-react";
import type { ReactNode } from "react";
import React from "react";
import {
  MdKeyboardControlKey,
  MdOutlineKeyboardCommandKey,
} from "react-icons/md";
interface Key {
  key: string;
  shortcut: string;
  technical_id: string;
  icon: () => ReactNode;
}

export function getCurrentOS(): OS {
  const userAgent = window.navigator.userAgent;

  if (userAgent.includes("Win")) {
    return "Windows";
  }
  if (userAgent.includes("Mac")) {
    return "MacOS";
  }
  if (userAgent.includes("Linux")) {
    return "Linux";
  }
  return "Unknown";
}

export default function getControlOrCommandKey(): Key {
  const os = getCurrentOS();
  if (os === "MacOS") {
    return {
      key: "Command",
      shortcut: "cmd",
      technical_id: "Meta",
      icon: () =>
          React.createElement(Command, {
          size: 11.5,
          style: {
            marginTop: "1px"
          }
    })
  }
}
  return {
    key: "Control",
    shortcut: "Ctrl",
    technical_id: "Control",
    icon: () => React.createElement(ChevronUp, {
          size: 11.5,
          style: {
            marginTop: "1px"
          }
    }),
  };
}
