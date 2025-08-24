import type React from "react";
import { useEffect, useRef } from "react";

type UploaderProps = {
	setUpload: React.Dispatch<React.SetStateAction<boolean>>
}

export default function UploaderModal({ setUpload }: UploaderProps) {
	const dialogRef = useRef<HTMLDialogElement>(null);

	useEffect(() => {
		dialogRef.current?.showModal()
	})

	return (
		<dialog ref={dialogRef} className="rounded-md w-md bg-black/30 backdrop-blur-xs top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 border-1 border-teal-600 h-50">
			<button onClick={() => setUpload(false)} 
							className="">
				<i className="fa-solid fa-xmark" />
			</button>
			<p> this is a dialog window </p>
		</dialog>
	)
}
