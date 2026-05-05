use qrate::{ Generator, QBank, SBank, SQLiteDB };

fn main() -> Result<(), String>
{
    let sbank = load_students().ok_or("No Students DB!".to_string())?;
    let qbank = load_questions().ok_or("No Questions DB!".to_string())?;
    let generator = Generator::new(&qbank, 1, 51, 10, &sbank).ok_or("Index Error!")?;
    generator.save_shuffled_exams("./IS.path".to_string(), "txt")?;
    generator.save_shuffled_exams("./IS.path".to_string(), "docx")?;
    generator.save_shuffled_exams("./IS.path".to_string(), "pdf")?;
    generator.save_shuffled_exams("./IS.path".to_string(), "hwpx")?;
    generator.save_shuffled_exams("./IS.path".to_string(), "hwp")
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