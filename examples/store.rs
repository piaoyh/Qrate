
use std::fs::File;
use std::io::{ Error, Write };
use qrate::{ Generator, QBank, SBank, SQLiteDB };

fn main() -> Result<(), String>
{
    let sbank = load_students().ok_or("No Students DB!".to_string())?;
    let qbank = load_questions().ok_or("No Questions DB!".to_string())?;
    let generator = Generator::new(&qbank, 1, 51, 10, &sbank).ok_or("Index Error!")?;
    generator.save_shuffled_exams("./IS_exam".to_string(), "txt")?;
    generator.save_shuffled_exams("./IS_exam".to_string(), "docx")?;
    // generator.save_shuffled_exams("./IS_exam".to_string(), "pdf")?;
    generator.save_shuffled_exams("./IS_exam".to_string(), "hwpx")?;
    generator.save_shuffled_exams("./IS_exam".to_string(), "hwp")?;

    let txt = generator.export_shuffled_exams_in_txt();
    let mut file = File::create("./IS_exam_export.txt".to_string()).map_err(|e: Error| e.to_string())?;
    file.write_all(txt.as_slice()).map_err(|e: Error| e.to_string())?;

    let docx = generator.export_shuffled_exams_in_docx().unwrap();
    let mut file = File::create("./IS_exam_export.docx".to_string()).map_err(|e: Error| e.to_string())?;
    file.write_all(docx.as_slice()).map_err(|e: Error| e.to_string())?;

    Ok(())
}

fn load_students() -> Option<SBank>
{
    use qrate::SBDB;
    SQLiteDB::open("./Students".to_string())?.read_sbank()
}

fn load_questions() -> Option<QBank>
{
    use qrate::QBDB;
    SQLiteDB::open("./Information_Security".to_string())?.read_qbank()
}