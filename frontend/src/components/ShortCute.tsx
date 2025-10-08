import { LucideCommand, LucidePlus, LucideSend } from "lucide-react";
import { useEffect } from "react";

export interface ShortcutProps {
    targetInput: string[];
}

const targetTo: Record<string, React.ReactElement> = {
    "Meta": <LucideCommand/>,
    "K": <p> K </p>,
    "Enter": <LucideSend/>
}

export default function ShortcutButton(props: ShortcutProps) {
    const cancelInput = "Escape";

    useEffect(() => {},[])


    return <div> {targetTo["Meta"]} <LucidePlus/> {targetTo["Enter"]}</div>
}