use qrate::{ ErrorMessage, QBank, QBDB, SQLiteDB, Excel, Spreadsheet };


fn main() -> Result<(), String>
{
    let qbank = load_questions().expect("No Questions DB!");
    let _ = match Excel::open("./output".to_string())
    {
        Ok(mut excel) => excel.write_qbank(&qbank),
        Err(e) => panic!("Failed to open Excel file: {}", e),
    };
    let _ = match Spreadsheet::open("./output".to_string())
    {
        Ok(mut spreadsheet) => spreadsheet.write_qbank(&qbank),
        Err(e) => panic!("Failed to open Spreadsheet file: {}", e),
    };
    Ok(())
}

fn load_questions() -> Result<QBank, ErrorMessage>
{
    use qrate::QBDB;
    SQLiteDB::open("./Information_Security".to_string())?.read_qbank()
}