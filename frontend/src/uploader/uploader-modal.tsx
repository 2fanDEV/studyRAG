import type React from "react";
import { useEffect, useRef } from "react";
import FileInput from "../input";

type UploaderProps = {
	setUpload: React.Dispatch<React.SetStateAction<boolean>>
}

export default function UploaderModal({ setUpload }: UploaderProps) {
	const dialogRef = useRef<HTMLDialogElement>(null);

	useEffect(() => {
		dialogRef.current?.showModal()
	})

	return (
		<div>
		<dialog ref={dialogRef} className="flex flex-col rounded-md w-md bg-white/10 backdrop-blur-xs top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 border-1 border-teal-600 h-50">
			<button onClick={() => setUpload(false)}
				className="self-start">
				<i className="fa-solid fa-xmark" />
			</button>

				<FileInput/>
		</dialog>

		</div>
	)
}
