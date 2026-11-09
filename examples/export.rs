use std::fs::File;
use std::io::{ Error, Write };
use qrate::{ QBankExporter, QBank, SQLiteDB, ErrorMessage };

fn main() -> Result<(), String>
{
    let qbank = load_questions().expect("No Questions DB!");
    let exporter = QBankExporter::new(qbank,"Answer Sheet".to_string());
    exporter.save_qbank("./IS_qbank".to_string(), "txt", true)?;
    exporter.save_qbank("./IS_qbank".to_string(), "docx", true)?;
    // exporter.save_qbank("./IS_exam".to_string(), "pdf")?;
    exporter.save_qbank("./IS_qbank".to_string(), "hwpx", true)?;
    exporter.save_qbank("./IS_qbank".to_string(), "hwp", true)?;

    let txt = exporter.export_qbank_in_txt(true);
    let mut file = File::create("./IS_qbank.txt".to_string()).map_err(|e: Error| e.to_string())?;
    file.write_all(txt.as_slice()).map_err(|e: Error| e.to_string())?;

    let docx = exporter.export_qbank_in_docx(true).unwrap();
    let mut file = File::create("./IS_qbank.docx".to_string()).map_err(|e: Error| e.to_string())?;
    file.write_all(docx.as_slice()).map_err(|e: Error| e.to_string())?;

    Ok(())
}

fn load_questions() -> Result<QBank, ErrorMessage>
{
    use qrate::QBDB;
    SQLiteDB::open("./Information_Security".to_string())?.read_qbank()
}