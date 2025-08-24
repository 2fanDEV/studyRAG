
interface Pdf {
	file_name: String
}

function PdfIcon() {
	 return(
		 <div className="absolute size-14">
			 <img src="src/assets/pdf_icon.png"/>
		 </div>
	 )
}


export default function Pdf() {
	return (
		<div className="relative">
				<PdfIcon></PdfIcon>
		</div>
	)
}
