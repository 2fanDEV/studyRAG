import type { Model } from "@/types/models.d"
import { createContext, useContext } from "react"

export interface ModelContextType {
    models: Model[]
    setModels: (models: Model[]) => void
}

export const ListModelsContext= createContext<ModelContextType | undefined>(undefined)

export const useModels =  () => useContext(ListModelsContext);