import type { Draggable } from "../types/app";
import useAxios from "./useAxios";

export function saveDraggable(draggable: Draggable) {
    let {loading, sendRequest } = useAxios({
        method: "POST",
        url: import.meta.env.VITE_API +"/draggable/save",
        headers: {
            "Content-Type": "application/json",
        },       
    })
    sendRequest({data: draggable});
}

function loadAllDraggables() {

}