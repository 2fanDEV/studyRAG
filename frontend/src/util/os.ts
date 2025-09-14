
export type OS = "Windows" | "MacOS" | "Linux" | "Unknown";

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

export default function getAltOrCmdKey(): string {
  const os = getCurrentOS();
  if (os === "MacOS") {
 //   return "fa-solid fa-command";
    return "cmd";
  }
  return "alt";
}   

