use std::fs::File;
use std::io::{ Error, Write };
use qrate::{ SBankExporter, SBank, SQLiteDB, ErrorMessage };

fn main() -> Result<(), String>
{
    let sbank = load_students().expect("No Students DB!");
    let exporter = SBankExporter::new(sbank,"Student List".to_string());
    exporter.save_sbank("./students".to_string(), "txt")?;
    exporter.save_sbank("./students".to_string(), "docx")?;
    // exporter.save_sbank("./save_sbank".to_string(), "pdf")?;
    exporter.save_sbank("./students".to_string(), "hwpx")?;
    exporter.save_sbank("./students".to_string(), "hwp")?;

    let txt = exporter.export_sbank_in_txt();
    let mut file = File::create("./students.txt".to_string()).map_err(|e: Error| e.to_string())?;
    file.write_all(txt.as_slice()).map_err(|e: Error| e.to_string())?;

    let docx = exporter.export_sbank_in_docx().unwrap();
    let mut file = File::create("./students.docx".to_string()).map_err(|e: Error| e.to_string())?;
    file.write_all(docx.as_slice()).map_err(|e: Error| e.to_string())?;

    Ok(())
}

fn load_students() -> Result<SBank, ErrorMessage>
{
    use qrate::SBDB;
    SQLiteDB::open("./Students".to_string())?.read_sbank()
}