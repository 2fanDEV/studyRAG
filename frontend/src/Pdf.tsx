export interface PdfDocument {
	id: String | undefined,
	file_name: String
	page_length: number,
	length: number,
	blob: Number[],
	ty: String,
}

function PdfIcon() {
	 return(
		 <div className="absolute size-14">
			 <img src="src/assets/pdf_icon.png"/>
		 </div>
	 )
}


export default function Pdf(p0: { id: undefined; file_name: string; page_length: number; length: number; blob: Int8Array<ArrayBufferLike>; ty: string }) {
	return (
		<div className="relative">
				<PdfIcon></PdfIcon>
		</div>
	)
}
