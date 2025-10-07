import { useState } from "react";
import type { Model } from "./types/models.d";
import { DraggableCanvas } from "./composition/DraggableCanvas";
import { Menu } from "./composition/Menu";

function App() {
  const [model, setModel] = useState<Model | undefined>(undefined);
  const [models, setModels] = useState<Model[]>([]);


  return <div>
    <Menu/>
    <DraggableCanvas/>
  </div>

}

export default App;
