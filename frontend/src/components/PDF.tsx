type PDFProps = {
  name: string;
  ty: string;
  path: string;
  len: number;
};

export default function PDF(props: PDFProps) {
  return (
    <div>
      <div className="flex justify-center">
        <img src="/public/pdf_icon.png" className="w-15 h-15" />
      </div>
      <p> {props.name} </p>
    </div>
  );
}
