import { useCallback } from "react";
import useAxios from "../hooks/useAxios";
import type { Draggable } from "../types/app";

export default function useGetAllDraggables()  {
    const {sendRequest, ...misc} = useAxios<Draggable[]>( {
        url: import.meta.env.VITE_API + "/draggable/getAll",
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "Accept": "application/json"
        }
    });
    
    const getAllDraggables = useCallback(()   => {
        return sendRequest();
    }, [sendRequest])
    return {getAllDraggables, ...misc};
}

export function useSaveDraggable() {
    const {sendRequest, ...misc} = useAxios({
        url: import.meta.env.VITE_API + "/draggable/save",
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Accept": "application/json",
        },
    });

    const saveDraggable = useCallback((draggable: Draggable) => {
            sendRequest({ data: draggable });
    },[sendRequest])

    return {saveDraggable, ...misc};
}