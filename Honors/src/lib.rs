use docx_rs::*;

pub fn merge_docx(input: String) -> Result<(), DocxError> {
    let path = std::path::Path::new("./output.docx");
    let file = std::fs::File::create(&path).unwrap();

    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(input)))
        .build()
        .pack(file)?;

    Ok(())
}