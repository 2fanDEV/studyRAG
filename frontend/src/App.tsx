import { useState } from 'react'
import './App.css'
import { DndContext, type UniqueIdentifier } from '@dnd-kit/core'
import PDF from './components/PDF'

type Position = { x: number, y: number };
type Positions = Record<UniqueIdentifier, Position>;

function App() {

	const [positions, setPositions] = useState<Positions>({
		a: { x: 0, y: 0 },
		b: { x: 100, y: 0 }
	});
	return (
		<>
			<DndContext onDragEnd={({ delta, active }) => {
				setPositions(prev => {
					const id = active.id as string;
					const pos = prev[id];
					return {
						...prev,
						[id]: {
							x: pos.x + delta.x,
							y: pos.y + delta.y
						}
					}
				})
			}
			}>
			{
				Object.entries(positions).map(([id, pos]) => {
						return (<PDF id={id} position={pos}/>)
				})
			}
			</DndContext >
		</>
	)
}

export default App
