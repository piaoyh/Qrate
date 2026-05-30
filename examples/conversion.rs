
use qrate::{ Header, Question, QBank, Choices };
use qrate::{ Student, SBank };

fn main()
{
    make_student_db();
    make_qbank_db();
}

fn make_student_db()
{
    use qrate::{ Excel, SBDB, SQLiteDB };
    
    let mut students = SBank::new();
    students.push_student(Student::new("Антон Алексеевич Куликов".to_string(), "1".to_string()));
    students.push_student(Student::new("Артём Андреевич Слётов".to_string(), "2".to_string()));
    students.push_student(Student::new("Ислам Элгарович Мамбетказиев".to_string(), "3".to_string()));
    students.push_student(Student::new("Ханыль  Лим".to_string(), "4".to_string()));

    let mut db = SQLiteDB::open("./Students".to_string()).unwrap();
    if let Err(e) = db.write_sbank(&students)
        { println!("error = {}", e); }

    let mut db = Excel::open("./Students".to_string()).unwrap();
    if let Err(e) = db.write_sbank(&students)
        { println!("error = {}", e); }
}

fn make_qbank_db()
{
    use qrate::{ Excel, QBDB, SQLiteDB };

    let vector = vec![
        (OldQuestionBank::new_is().convert_old_into_new(), "./Information_Security"),
        (OldQuestionBank::new_cprog().convert_old_into_new(), "./C_Programming"),
        (OldQuestionBank::new_oop().convert_old_into_new(), "./CPP_Programming"),
        (OldQuestionBank::new_dsa().convert_old_into_new(), "./Data_Structure_and_Algorithms"),
        (OldQuestionBank::new_se().convert_old_into_new(), "./Software_Engineering"),
        (OldQuestionBank::new_mis().convert_old_into_new(), "./Management_Information_Systems"),
    ];

    for (qbank, path) in vector
    {
        let mut db = SQLiteDB::open(path.to_string()).unwrap();
        if let Err(e) = db.write_qbank(&qbank)
            { println!("error = {}", e); }

        let mut db = Excel::open(path.to_string()).unwrap();
        if let Err(e) = db.write_qbank(&qbank)
            { println!("error = {}", e); }
    }
}


const NUMBER_CHOICES: usize = 4;
const SAVE_PAPER_SPACE: &str = "";  // "\t"

/// For backward compatibility. It is temporary.
#[derive(Clone)]
pub struct OldQuestion
{
    pub question: &'static str,
    pub choice: [&'static str; NUMBER_CHOICES],
    #[allow(dead_code)] pub group: u16,    // 1-based
    pub answer: u8,
}

impl OldQuestion
{
    pub fn question(&self) -> &str          { self.question }
    #[allow(dead_code)] pub fn group(&self) -> u16              { self.group }
    pub fn answer(&self) -> (u8, u8)        { (self.answer & 0x0f, self.answer >> 4) }
    pub fn has_multianswers(&self) -> bool  { self.answer > NUMBER_CHOICES as u8}
    pub fn choice(&self, num: u8) -> &str   { self.choice[(num - 1) as usize] }
    pub fn make(question: &'static str,
                choice: [&'static str; NUMBER_CHOICES],
                group: u16,
                answer: u8) -> OldQuestion
    {
        OldQuestion { question, choice, group, answer }
    }
}

/// Represents a Question Bank, containing a header and a vector of questions.
#[derive(Clone)]
pub struct OldQuestionBank
{
    pub header: Header,
    pub questions: Vec<OldQuestion>,
}

impl OldQuestionBank
{
    pub fn new() -> Self
    {
        Self
        {
            header: Header::new_empty(),
            questions: Vec::new(),
        }
    }

    pub fn convert_old_into_new(&self) -> QBank
    {
        let old_questions = &self.questions;
        let mut new_questions = Vec::<Question>::new();
        let mut id = 1_u16;
        for old_question in old_questions
        {
            let mut question = Question::new_empty();
            question.set_id(id);
            question.set_group(old_question.group());
            question.set_category(if old_question.has_multianswers() {2} else {1});
            question.set_question(old_question.question().to_string());
            let mut choices = Choices::new();
            for i in 1..=4
            {
                let choice = old_question.choice(i).to_string();
                let is_answer = (old_question.answer().0 == i) || (old_question.answer().1 == i);
                choices.push((choice, is_answer));
            }
            question.set_choices(choices);
            new_questions.push(question);
            id += 1;
        }
        let mut qbank = QBank::new_with_header(self.header.clone());
        qbank.set_questions(new_questions);
        qbank
    }

    // pub fn get_header(st: &Student) -> String
    // {
    //     let header = "2nd & Final Examinations - Information Security\n".to_string()
    //                 + &format!("Name: {}\tID: {}\n", st.get_name(), st.get_id())
    //                 + "Notice:\n"
    //                 + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
    //                 + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
    //                 + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
    //                 + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
    //                 + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
    //                 + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
    //                 + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
    //                 + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
    //                 + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
    //                 + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n"
    //                 + "* The questions 1 ~ 5 belong to the 2nd midterm exam and the questions 6 ~ 25 belong to the Final exam.\n\n";
    //     return header;
    // }

    #[allow(dead_code)]
    pub fn count_questions(&mut self) -> usize
    {
        self.questions.len()
    }

    pub fn new_is() -> Self
    {
        let header = Header::new(
            "Information Security".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
        
        let mut questions = Vec::<OldQuestion>::new();

        let mut num = 1u16;
        let qb = OldQuestion
        {
            question: r#"Which one of the following is incorrect?   ……. (   )"#,
            choice: [r#"Cryptography can be viewed as one of the tools to achieve security."#,
                    r#"Cryptography is a tool to prevent eavesdropping."#,
                    r#"Masquerading means sending a message that was not sent."#,
                    r#"Modification or interception means changing the contents of a message."#],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;   // num = 2
        let qb = OldQuestion
        {
            question: "Which one of the following is not one of the three elements of security?   ……. (   )",
            choice: ["Confidentiality",
                    "Integrity",
                    "Availability",
                    "Access Control"],
            group: num,
            answer: 4,
        };
        questions.push(qb); 

        num += 1;   // num = 3
        let qb = OldQuestion
        {
            question: "Which ones of the following are incorrect?   ……. (   )",
            choice: ["Confidentiality means that none who have proper authority can access the information.",
                    "Integrity means that the information has not been modified or masqueraded.",
                    "Availability means that service is provided continuously without stopping.",
                    "Access Control means managing access to assets according to the user's authority."],
            group: num,
            answer: 1,
        };
        questions.push(qb); 

        num += 1;  // num = 4
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect?   ……. (   )",
            choice: ["Confidentiality can be achieved by Encryption / Decryption.",
                    "Integrity can be achieved by Encryption + CBC mode, or Hash.",
                    "Availability can be achieved by Cryptanalysis.",
                    "Identification and Authentication can be achieved by ID/PW, SMS verification, OTP device, etc."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 5
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect?   ……. (   )",
            choice: ["Identification and Authentication means Verifying who the user is and what privileges they have.",
                    "Access control can be achieved by Role-based access control methods, etc.",
                    "If only integrity is of interest, hash can be used.",
                    "Decryption ensures confidentiality, and Encryption can ensure even availability."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 6
        let qb = OldQuestion
        {
            question: "Which one of the following is not about Encryption? ……. (   )",
            choice: ["In cryptography, encryption is the process of encoding information.",
                    "This process converts the original representation of the information, known as plaintext, into an alternative form known as ciphertext.",
                    "Ideally, only authorized parties can decipher a ciphertext back to plaintext and access the original information.",
                    "Encryption prevents interference and also denies the intelligible content to a would-be interceptor."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 7
        let qb = OldQuestion
        {
            question: "Which ones of the following are incorrect? ……. (   ,   )",
            choice: ["In cryptography, plaintext usually means unencrypted information pending input into cryptographic algorithms, usually encryption algorithms.",
                    "Plaintext usually refers to data that is transmitted or stored encrypted.",
                    "In cryptography, ciphertext or cyphertext is the result of encryption performed on plaintext using an algorithm, called a cipher.",
                    "Ciphertext usually refers to data that is transmitted or stored unencrypted."],
            group: num,
            answer: 0x24,
        };
        questions.push(qb);

        num += 1;  // num = 8
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT describing Kerckhoffs’s Principle and Shannon's maxim? ....... (   )",
            choice: ["The design of a cryptosystem should not be publicized but be kept secret.",
                    "Kerckhoffs’s Principle holds that a cryptosystem should be secure, even if everything about the system, except the key, is public knowledge.",
                    "The enemy knows the system, i.e., one ought to design systems under the assumption that the enemy will immediately gain full familiarity with them.",
                    "The security of a cryptosystem should depend only on the key."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 9
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT correct? ……. (   )",
            choice: ["There are three encryption ways: substitution, transposition, and product.",
                    "Transposition encryption is shuffling the positions of symbols.",
                    "Product encryption is XOR (exclusive OR) operation of one symbol with another symbol.",
                    "Substitution encryption is replacing one symbol with another symbol."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 10
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT correct? ……. (   )",
            choice: ["KEY should be shared with everybody for security.",
                    "Product encryption is a combination of transposition and substitution.",
                    "Transposition encryption is easy to attack.",
                    "Substitution encryption can be cryptanalyzed with frequency analysis."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 11
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect? ……. (   )",
            choice: ["As computers have been developed, a lot of attempts to cryptanalyze have never been taken with strong computing power.",
                    "There are two definitions of SECURE cryptography: Unconditional Security, and Conditional Security.",
                    "Unconditional Security is also called Information-theoretical Security.",
                    "Conditional Security is also called Computational Security."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 12
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT about Unconditional Security? ……. (   )",
            choice: ["It is assumed that cryptanalysts can use unlimited computing power.",
                    "If cryptanalysts cannot break the cryptographic algorithm even with unlimited computing power, the cryptographic algorithm is called UNCONDITIONALLY SECURE.",
                    "The cryptanalysts can decrypt the ciphertexts if they can do everything.",
                    "The cryptanalysts cannot get any hint from the ciphertexts."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 13
        let qb = OldQuestion
        {
            question: "Which ones of the following are NOT about Conditional Security? ……. (   )",
            choice: ["If it takes unrealistic time for the cryptanalysts to break the cryptographic algorithm, even though they use the best way known so far in order to break it, it is called Conditional Security.",
                    "It requires the key that has the same length as that of plaintext.",
                    "If it is conditionally secure, it is also said to be secure computationally.",
                    "The cryptanalysts can decrypt the ciphertexts if they have enough time."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 14
        let qb = OldQuestion
        {
            question: "Which ones of the following describe One Time Pad? ……. (   ,   ) ",
            choice: ["The key has a shorter length than that of the plaintext.",
                    "Encryption with XOR operation.",
                    "The length of the ciphertext, the length of the key, and the length of the plaintext are all the same.",
                    "The key can be chosen according to your mood."],
            group: num,
            answer: 0x23,
        };
        questions.push(qb);

        num += 1;  // num = 15
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT characteristic of unconditional security? ……. (   )",
            choice: ["It is unconditionally secure if the different key is used for every encryption.",
                    "If the plaintext is long, the key should be long too.",
                    "Encryption of a 1 TB hard disk requires a 1 TB key.",
                    "Practically, it is easy to guarantee perfect randomness."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 16
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of conditional security? ……. (   )",
            choice: ["It is considered to be insecure if it takes long enough to cryptanalyze.",
                    "It should be assumed that the encryption mechanism of the encryption algorithm is published.",
                    "The security of the encryption algorithm should depend only on the key.",
                    "The most basic and simplest attack that attackers can use is the brute force attack."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 17
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT about computational security? ……. (   )",
            choice: ["It is considered to be secure if the brute force attack to the key takes long enough.",
                    "“Long enough” in the expression “it takes long enough to break the cryptographic algorithm” can mean different things depending on the situation, but usually it means longer than human life span.",
                    "When a password is created at a website, the minimum length of password and combination have become shorter and simpler than before.",
                    "It is recommended to change the password periodically."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 18
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to cryptanalytic attack? ……. (   )",
            choice: ["Plaintext-Only Attack (POA)",
                    "Known-Plaintext Attack (KPA)",
                    "Chosen-Plaintext Attack (CPA)",
                    "Chosen-Ciphertext Attack (CCA)"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 19
        let qb = OldQuestion
        {
            question: "Which one of the following is the characteristics of Ciphertext-Only Attack (COA)? .. (   ,   )",
            choice: ["An attacker has only a ciphertext C.",
                    "It is the most common type and the most weak attack.",
                    "An attacker knows some pairs of plaintext and ciphertext except the ciphertext C.",
                    "It is assumed that the pairs are randomly acquired by the attacker."],
            group: num,
            answer: 0x12,
        };
        questions.push(qb);

        num += 1;  // num = 20
        let qb = OldQuestion
        {
            question: "Which one of the following is the characteristics of Known-Plaintext Attack (KPA)? .. (   ,   )",
            choice: ["An attacker can ask for the corresponding ciphertexts of the chosen plaintexts.",
                    "In other words, an attacker can get the ciphertexts from the chosen plaintexts.",
                    "An attacker knows some pairs of plaintext and ciphertext except the ciphertext C.",
                    "It is assumed that the pairs are randomly acquired by the attacker."],
            group: num,
            answer: 0x34
        };
        questions.push(qb);

        num += 1;  // num = 21
        let qb = OldQuestion
        {
            question: "Which one of the following is the characteristics of Chosen-Plaintext Attack (CPA)? . (   ,   )",
            choice: ["An attacker has only a ciphertext C.",
                    "An attacker can ask for the corresponding ciphertexts of the chosen plaintexts.",
                    "In other words, an attacker can get the ciphertexts from the chosen plaintexts.",
                    "It is assumed that the pairs are randomly acquired by the attacker."],
            group: num,
            answer: 0x23,
        };
        questions.push(qb);

        num += 1;  // num = 22
        let qb = OldQuestion
        {
            question: "Which one of the following is the characteristics of Chosen-Ciphertext Attack (CCA)? .. (   )",
            choice: ["An attacker can encrypt plaintexts and decrypt ciphertexts except the ciphertext C.",
                    "This attack is stronger than CPA, KPA, and COA.",
                    "An attacker knows some pairs of plaintext and ciphertext except the ciphertext C.",
                    "An attacker can ask for the corresponding ciphertexts of the chosen plaintexts."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 23
        let qb = OldQuestion
        {
            question: " Which one of the following is incorrect? ……. (   )",
            choice: ["Substitution is replacing the symbols of the plaintext with the different symbols according to a certain rule so as to create a ciphertext.",
                    "Transposition is changing the order of the bits of plaintext according to a certain rule so as to create a ciphertext.",
                    "Unconditional secure cryptography cannot be cryptanalyzed by attackers though they have unlimited computing power.",
                    "One Time Pad cryptography does not satisfy the requirement of unconditional secure cryptography."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 24
        let qb = OldQuestion
        {
            question: "Which ones of the following are incorrect? ……. (   ,   )",
            choice: ["Implementation of unconditional security is virtually impossible so that computational secure cryptography is used instead.",
                    "Unconditional security is so possible that computational secure cryptography is only theoretical and impractical.",
                    "If cryptanalysis is not completed even with long enough time, it is called computationally secure.",
                    "If cryptanalysis is not completed even with long enough time, it is called absolutely secure."],
            group: num,
            answer: 0x24,
        };
        questions.push(qb);

        num += 1;  // num = 25
        let qb = OldQuestion
        {
            question: " Which one of the following is incorrect? ……. (   )",
            choice: ["In symmetric key cryptography, the encryption key is the SAME as the  decryption key.",
                    "In asymmetric key cryptography, the encryption key is NOT the same as the decryption key.",
                    "The theoretically best key is the randomly chosen key that has the same length as the plaintext.",
                    "Practically, it is required to generate such a key that has the double length of the plaintext because the data to be encrypted is so big."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 26
        let qb = OldQuestion
        {
            question: " Which one of the following is incorrect? ……. (   )",
            choice: ["It is good if the encryption algorithm uses a separate key generator.",
                    "The random numbers generated by a key generator are called ‘pseudorandom numbers’ and are theoretically impossible to distinguish from real random numbers.",
                    "In block encryption, it is to encrypt a plaintext bit by bit.",
                    "Both block encryption and stream encryption require a key."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 27
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe block encryption? ……. (   )",
            choice: ["The lengths of the plaintext, the key, and the ciphertext are all the same.",
                    "It divides a plaintext into blocks that have a predetermined size.",
                    "It encrypts the plaintext block by block.",
                    "The length of a plaintext to be encrypted has to be divisible by the length (size) of the block because the block encryption works block by block."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 28
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe stream encryption? ……. (   )",
            choice: ["It encrypts a plaintext bit by bit.",
                    "The lengths of the plaintext, the key, and the ciphertext are all the same.",
                    "If the length of a plaintext is ten bytes and the size of the block is four bytes, we have to append two dummy bytes to the plaintext in order to make the last block of four bytes.",
                    "Encryption and Decryption are done by XOR bit operation."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 29
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe stream encryption? ……. (   )",
            choice: ["The encryption unit is block.",
                    "Plaintext XOR Key = Ciphertext",
                    "Ciphertext XOR Key = Plaintext",
                    "If a key can be generated randomly, the steam encryption can be called the most secure encryption because it is the same as a one-time pad."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 30
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect? ……. (   )",
            choice: ["The encryption unit of block encryption is block while the encryption unit of stream encryption is bit.",
                    "Block encryption is comparatively faster than stream encryption.",
                    "The block encryption is easy to find whether it has been attacked because the whole block is affected if one bit is compromised while the stream encryption is hard to find whether it has been attacked even if one bit is compromised.",
                    "The block encryption is widely used for encryption of general data while the stream encryption is widely used for encryption of real-time services such as audio streaming and video streaming."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 31
        let qb = OldQuestion
        {
            question: "In 1972, NBS (currently, NIST) announced competition for a standard encryption algorithm. Which one of the following does not belong to the requirement list? ……. (   )",
            choice: ["It should be able to guarantee a high level of security.",
                    "Its definition should be complete and simple to understand.",
                    "Its security should depend on the secrecy of its algorithm.",
                    "It should be available for both users and manufacturers."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 32
        let qb = OldQuestion
        {
            question: "In 1972, NBS (currently, NIST) announced competition for a standard encryption algorithm. Which one of the following does not belong to the requirement list? ……. (   )",
            choice: ["Its application should be diverse.",
                    "It should be complicated to manufacture as an electronic device and to use",
                    "Its developers should cooperate in algorithm validation.",
                    "It should be allowed to be exported to other countries."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 33
        let qb = OldQuestion
        {
            question: "Which one of the following is the basic structure of DES? ……. (   )",
            choice: ["Lucifer structure",
                    "ShiftLeft structure",
                    "Permutation structure",
                    "Feistel structure"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 34
        let qb = OldQuestion
        {
            question: "How many rounds does DES use the Feistel structure? ……. (   )",
            choice: ["16",
                    "32",
                    "48",
                    "64"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 35
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about DES? ……. (   ,   )",
            choice: ["Avalanche effect − A small change in plaintext results in a very great change in the ciphertext.",
                    "Completeness − Each bit of ciphertext depends on many bits of plaintext.",
                    "One-to-one effect – A small change in plaintext results in a very small change in the ciphertext.",
                    "Sharpness − Each bit of ciphertext depends on each bit of plaintext."],
            group: num,
            answer: 0x12,
        };
        questions.push(qb);

        num += 1;  // num = 36
        let qb = OldQuestion
        {
            question: "Which one of the following are incorrect about DES? ……. (   )",
            choice: ["DES was developed as an American standard for symmetric key cryptographic algorithms.",
                    "DES has the same encryption and decryption process.",
                    "DES substitutes the keys used in the encryption process in reverse order when decrypting.",
                    "DES can be implemented only in hardware."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 37
        let qb = OldQuestion
        {
            question: "Which one of the following describes weak keys of DES correctly? ….… (   ,   )",
            choice: ["When a weak key is used, if a plaintext is encrypted twice with the same key, the plaintext is gotten back.",
                    "A weak key pair is a pair of one key and the other key.",
                    "The key that has such a pattern that it is not affected by transposition of key schedulers of DES creates such problems.",
                    "When a weak key pair is used, if a plaintext is encrypted with one of a pair and then encrypted with the other of the pair again, the plaintext is gotten back."],
            group: num,
            answer: 0x13,
        };
        questions.push(qb);

        num += 1;  // num = 38
        let qb = OldQuestion
        {
            question: "Which ones of the following cryptographic algorithms are still secure against modern computing power? ……. (   ,   )",
            choice: ["DES",
                    "double DES",
                    "triple DES",
                    "RSA"],
            group: num,
            answer: 0x34,
        };
        questions.push(qb);

        num += 1;  // num = 39
        let qb = OldQuestion
        {
            question: "Which ones of the following describe the problems of the symmetric-key cryptographic algorithms? ……. (   ,   )",
            choice: ["You have key distribution problems.",
                    "You have speed problems for encryption and decryption.",
                    "You have to get a repository for your keys.",
                    "You have key management problems."],
            group: num,
            answer: 0x14,
        };
        questions.push(qb);

        num += 1;  // num = 40
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT about communication using the public-key cryptographic algorithms? ……. (   )",
            choice: ["It requires a pair of keys: encryption key and decryption key.",
                    "A public key is used for decryption while a private key is used for encryption.",
                    "Everybody can lock but only the one who has the private key can open.",
                    "RSA is one of the examples of communication using the public-key cryptographic algorithms."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 41
        let qb = OldQuestion
        {
            question: "Which ones of the following describe the strength of public-key cryptographic algorithms? ……. (   ,   )",
            choice: ["It is comparatively fast in both encryption and decryption.",
                    "Since all you have to do is to publish your public key at a predetermined repository, you don’t have to worry about key distribution.",
                    "In order to have cryptographic communication, each one should have only two keys: a private key and a public key.",
                    "Since private keys can be easily calculated and derived from public keys, you can recover your private key easily when you lose your private key."],
            group: num,
            answer: 0x23,
        };
        questions.push(qb);

        num += 1;  // num = 42
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe RSA? ……. (   )",
            choice: ["It is widely considered as the best Public-Key Cryptographic Algorithm in the world, invented in 1977.",
                    "RSA is the initials of the three inventors: Rivest, Shamir, Adleman.",
                    "So far, it has been the most widely used public key cryptographic algorithm in the world.",
                    "With the advent of RSA, the field of use of cryptography has expanded from military to civilian."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 43
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe RSA? ……. (   )",
            choice: ["The public key is composed of two numbers: one is a small prime number and the other is the product of two prime numbers.",
                    "The harder the factorization is, the securer RSA is.",
                    "In reality, RSA uses a 2048-bit number as the product of two numbers.",
                    "It is logic operation oriented rather than mathematical operation oriented."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 44
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe a Hash function? ……. (   )",
            choice: ["A hash function is a mathematical function that converts an input of arbitrary length into an output of fixed length.",
                    "The length of the output of the hash function is always the same and does not depend on the length of the input to the hash function.",
                    "The length of the output of the recent hash functions is mainly 512 bits. ",
                    "Therefore, hash functions can be said to be the same as general compression functions."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 45
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect? ……. (   )",
            choice: ["The compression function can restore the original one after compression while the output of the hash function cannot restore the original one after compression.",
                    "The compression function is lossless while the hash function is lossy.",
                    "The size of the output of the compression function depends on the size of the input to the compression function while the size of the output of the hash function is always the same regardless of the size of the input to the hash function.",
                    "The output of the hash function is almost the unique value of the input to the hash function so that it can be a fingerprint."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 46
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe digital signature? ……. (   )",
            choice: ["The information in digital form, which is used to verify the signer and verify that the signer has signed a specific digital document.",
                    "Digital signature is an image of a person's signature.",
                    "A digital signature is a ciphertext generated using a public key cryptographic algorithm.",
                    "Digital signatures are guaranteed to have the same effect of traditional  signatures and seals."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 47
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe the structure of an Accredited Certificate for digital signature? ……. (   )",
            choice: ["Version: Classification of the format of the certificate. Most public certificates we use are version 3.",
                    "Serial number: Certificate serial number within the certification authority that issued the certificate.",
                    "Validity period (start, end): The period during which the certificate can be used, recording the start date and expiration date, in days.",
                    "Signature Algorithm: The algorithm used for issuing the certificate."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 48
        let qb = OldQuestion
        {
            question: "Which one of the following does describe RSA and digital signature correctly? ……. (   )",
            choice: ["In RSA, encryption is done with a public key and decryption is done with a private key for encryption/decryption, while encryption is done with a private key and decryption is done with a public key for verification in digital signature.",
                    "In RSA, encryption is done with a private key and decryption is done with a public key for encryption/decryption, while encryption is done with a public key and decryption is done with a private key for verification in digital signature.",
                    "In RSA, encryption is done with a public key and decryption is done with a private key for verification, while encryption is done with a private key and decryption is done with a public key for encryption/decryption in digital signature.",
                    "In RSA, encryption is done with a private key and decryption is done with a public key for verification, while encryption is done with a public key and decryption is done with a private key for encryption/decryption in digital signature."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 49
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe the randomness used for cryptography? ……. (   )",
            choice: ["The encryption algorithm must have randomness so as not to reveal additional information about the plaintext to the attacker from the ciphertext.",
                    "Since it must be regular for decryption, the randomness of the encryption algorithm must be random only in appearance and must be regular in reality.",
                    "Injecting randomness into cryptography enhances the security in the way to perform transposition.",
                    "Cryptographic algorithms in use today provide randomness before encryption."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 50
        let qb = OldQuestion
        {
            question: "Which one of the following is the wrong description of steganography? ……. (   )",
            choice: ["Steganography is a way to hide secret information by embedding it in an audio, video, image or text file.",
                    "It is one of the methods employed to protect classified or sensitive data from malicious attacks.",
                    "Steganography, often known as cover writing, is a technique for transforming a secret process into a fake-looking message.",
                    "Steganography alters the data structure of the plain text by using transposition and substitution."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 51
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe steganography? ……. (   )",
            choice: ["Steganography can be used for text, audio, video, or images.",
                    "Steganography technique is one of the cryptanalysis techniques.",
                    "Steganography is a method of concealing information within other data.",
                    "Steganography is an encryption technology that, when used with cryptography, provides an additional layer of data security."],
            group: num,
            answer: 2,
        };
        questions.push(qb);
        Self { header, questions }
    }

    pub fn new_cprog() -> Self
    {
        let header = Header::new(
            "C Programming".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
        
        let mut num = 1u16;
        let mut questions = Vec::<OldQuestion>::new();
        let mut qb = OldQuestion
        {
            question: r#"Which ones of the following are not correct?  …… (   ,   )
            #include <stdio.h>
            int main();
            {    // This is my first program.
                printf("Hello, World!\n")
                return 0;
            }"#,
            choice: [r#"int main();"#,
                    r#"{    // This is my first program"#,
                    r#"printf("Hello, World!\n")"#,
                    r#"return 0;"#],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;   // num = 2
        qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C language grammar?  …… (   )",
            choice: [";            is the end mark of the C statement.",
                    "/* Comment */        is a multi-line comment.",
                    "// Comment        is a single line comment.",
                    "A ~ Z, a ~ z, _, # and numbers 0 ~ 9 can all be used for variable names."],
            group: num,
            answer: 4,
        };
        questions.push(qb); 

        num += 1;  // num = 3
        qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C language grammar?  …… (   )",
            choice: ["C is case-sensitive.",
                    "Whitespaces in C language are blank, tab, new line character.",
                    "A multi-line comment starts with the characters */ and terminates with /*",
                    "You cannot have comments within comments and they do not occur within a string or character literals."],
            group: num,
            answer: 0x3,
        };
        questions.push(qb);

        num += 1;  // num = 4
        qb = OldQuestion
        {
            question: "Which one of the following cannot be a variable in C language?  …… (   )",
            choice: ["abc",
                    "0x123",
                    "a_123",
                    "_temp"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 5
        qb = OldQuestion
        {
            question: "Which one of the following can you use as a variable name in C language?  …… (   )",
            choice: ["if",
                    "int",
                    "for",
                    "until"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 6
        qb = OldQuestion
        {
            question: "Which one of the following is not the same as below in C language?  …… (   )
            int a = 2 + 3;",
            choice: ["inta = 2 + 3;",
                    "int a=2+3;",
                    "int a = 2+3;",
                    "int a=2 + 3;"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 7
        qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C language grammar?  …… (   )",
            choice: ["char is a data type of one byte.",
                    "int is a data type of four bytes.",
                    "long long is a data type of eight bytes.",
                    "float is a data type of eight bytes."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 8
        qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C language grammar?  …… (   )",
            choice: ["char is the same as signed char.",
                    "long is a data type of four bytes under Windows and of eight bytes under Linux.",
                    "unsigned is the same data type as unsigned int.",
                    "unsigned float is used for only positive float numbers."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 9
        qb = OldQuestion
        {
            question: "Which ones of the following are correct in C language grammar?  …… (   ,   )",
            choice: ["Lvalue is changeable, mutable, and inconstant.",
                    "In the statement const int PI = 3.14159;, PI is an example of Rvalue.",
                    "Literal expressions such as 6, 1.414, ‘a’, “Hello” are examples of Lvalue.",
                    "Rvalue can be assigned a value to."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 10
        qb = OldQuestion
        {
            question: "Which one of the following is not correct in C language?  …… (   )",
            choice: ["0xf == 017 == 0b1111 == 15",
                    "0x19 == 031 == 0b11001 == 25",
                    "0x8 == 08 == 0b1000 == 8",
                    "0x1 == 01 == 0b1 == 1"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 11
        qb = OldQuestion
        {
            question: "Which ones of the following are correct?  …… (   ,   )",
            choice: ["One byte is eight bits.",
                    "1 KiB == 1000 B",
                    "1 MB == 1000 KB",
                    "1 GiB == 1024 KiB"],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 12
        qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C language grammar?  …… (   )",
            choice: ["Constants are immutable or unchangeable variables.",
                    "Constants can be any of the basic data types such as an integer constant, a floating constant, a character constant and so on.",
                    "Literals are the expression of values such as 1, 3.14, ‘a’, “hello world!” and so on.",
                    "Constants can be called Lvalue in some contexts."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 13
        qb = OldQuestion
        {
            question: r#"Which one of the following is the correct output of the C program below?  …… (   )
            #include <stdio.h>
            int main()
            {
                int    a = 10, b = 20;
                printf(“a = %d and b = %d\n”, ++a, b++);
                return 0;
            }"#,
            choice: ["a = 10 and b = 20",
                    "a = 11 and b = 20",
                    "a = 10 and b = 21",
                    "a = 11 and b = 21"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 14
        qb = OldQuestion
        {
            question: "Which one of the following is not correct in C language?  …… (   )",
            choice: ["(0 && 0) == 0",
                    "(1 && 0) == 1",
                    "(0 && 1) == 0",
                    "(1 && 1) == 1"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 15
        qb = OldQuestion
        {
            question: "Which one of the following is not correct in C language?  …… (   )",
            choice: ["(0 || 0) == 0",
                    "(1 || 0) == 1",
                    "(0 || 1) == 0",
                    "(1 || 1) == 1"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 16
        qb = OldQuestion
        {
            question: "Which one of the following is not correct in C language?  …… (   )",
            choice: ["!(0 && 0) == 0",
                    "!(1 || 0) == 0",
                    "!(0 && 1) == 1",
                    "!(1 || 1) == 0"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 17
        qb = OldQuestion
        {
            question: "Which one of the following is correct in C language?  …… (   )",
            choice: ["(0b1100 & 0b1010) == 0b1000",
                    "(0b1100 & 0b1010) == 0b1001",
                    "(0b1100 & 0b1010) == 0b0110",
                    "(0b1100 & 0b1010) == 0b1110"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 18
        qb = OldQuestion
        {
            question: "Which one of the following is correct in C language?  …… (   )",
            choice: ["(0b1100 | 0b1010) == 0b1000",
                    "(0b1100 | 0b1010) == 0b1001",
                    "(0b1100 | 0b1010) == 0b0110",
                    "(0b1100 | 0b1010) == 0b1110"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 19
        qb = OldQuestion
        {
            question: "Which one of the following is correct in C language?  …… (   )",
            choice: ["(0b1100 ^ 0b1010) == 0b1000",
                    "(0b1100 ^ 0b1010) == 0b1001",
                    "(0b1100 ^ 0b1010) == 0b0110",
                    "(0b1100 ^ 0b1010) == 0b1110"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 20
        qb = OldQuestion
        {
            question: "Which ones of the following are correct in C language?  …… (   ,   )",
            choice: ["~(0b1100 & 0b1010) == 0b1010",
                    "~(0b1100 | 0b1010) == 0b0001",
                    "~(0b1100 ^ 0b1010) == 0b0110",
                    "!(0b1100 & 0b1010) == 0b0000"],
            group: num,
            answer: 0x42
        };
        questions.push(qb);

        num += 1;  // num = 21
        qb = OldQuestion
        {
            question: "Which one of the following is incorrect in C language?  …… (   )",
            choice: ["(0b10010110 << 1) == 0b00101100",
                    "(0b10010110 >> 1) == 0b01001011",
                    "(4 << 1) == 4 * 2",
                    "(4 >> 1) == 4 * 2"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 22
        qb = OldQuestion
        {
            question: r#"Which one of the following is the result of the below?  …… (   )
            ((8 >> 2) == (8 / 4)) ? (1 + (2 >> 1)) : (2 - (1 << 1))"#,
            choice: ["0",
                    "1",
                    "2",
                    "3"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        
        num += 1;  // num = 23
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int a = 1, b = 2;
                a = b;
                b = a;
                printf(“a = %d and b = %d\n”, a, b);
                return 0;
            }"#,
            choice: ["a = 2 and b = 2",
                    "a = 1 and b = 1",
                    "a = 2 and b = 1",
                    "a = 1 and b = 2"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 24
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int a = 1, b = 2;
                int tmp = a;
                a = b;
                b = tmp;
                printf(“a = %d and b = %d\n”, a, b);
                return 0;
            }"#,
            choice: ["a = 2 and b = 2",
                    "a = 2 and b = 1",
                    "a = 1 and b = 2",
                    "a = 1 and b = 1"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 25
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int a = 1, b = 3, c = 2, d = 4;
                int m = (a > b) ? a : b;
                m = (m > c) ? m : c;
                m = (m > d) ? m : d;
                printf(“m = %d\n”, m);
                return 0;
            }"#,
            choice: ["m = 1",
                    "m = 2",
                    "m = 3",
                    "m = 4"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 26
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            void swap(int a, int b);
            int main()
            {
                int a = 1, b = 2;
                swap(a, b);
                printf(“a = %d and b = %d\n”, a, b);
                return 0;
            }
            void swap(int a, int b)
            {
                int tmp = a;
                a = b;
                b = tmp;
            }"#,
            choice: ["a = 2 and b = 2",
                    "a = 2 and b = 1",
                    "a = 1 and b = 2",
                    "a = 1 and b = 1"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 27
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int a = 1, b = 2;
                int c = (a + b) << 2;
                if (c < 0)
                    printf(“negative!\n”);
                else if (c == 0)
                    printf(“zero!\n”);
                else if (c > a + b)
                    printf(“c is 12.\n”);
                else
                    printf(“c is positive.\n”);
                return 0;
            }"#,
            choice: ["negative!",
                    "zero!",
                    "c is 12.",
                    "c is positive."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 28
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int a = 1, b = 2;
                int c = (a + b) >> 2;
                switch (c)
                {
                case -1:	printf(“negative!\n”);		break;
                case 0:	printf(“zero!\n”);		break;
                case 6:	printf(“c is positive.\n”);	break;
                default:		printf(“c is 12.\n”);		break;
                }
                return 0;
            }"#,
            choice: ["negative!",
                    "zero!",
                    "c is 12.",
                    "c is positive."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 29
        let qb = OldQuestion
        {
            question: "Which one of the following is not an infinite loop?  …… (   )",
            choice: [
    r#"    LABEL1:
            printf(“FOREVER\n”);
            return;
            goto LABEL1;"#,
    r#"    for (;;)
            printf(“FOREVER\n”);"#,
    r#"    while (1)
            printf(“FOREVER\n”);"#,
    r#"    do
            printf(“FOREVER\n”);
        while (1);"#],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 30
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int arr[] = { 1, 2, 3, 4, 5 };
                int a = arr[2]--;
                printf(“The answer is %d.\n”, arr[a]);
                return 0;
            }"#,
            choice: ["The answer is 3.",
                    "The answer is 4.",
                    "The answer is 5.",
                    "The answer is 6."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 31
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <stdio.h>
            int main()
            {
                int arr[] = { 1, 2, 3, 4, 5 };
                int* ptr = arr;
                ptr++;
                printf(“The answer is %d.\n”, arr[1] - *ptr);
                return 0;
            }"#,
            choice: ["The answer is 0.",
                    "The answer is 1.",
                    "The answer is 2.",
                    "The answer is 3."],
            group: num,
            answer: 1,
        };
        questions.push(qb);
        Self { header, questions }
    }

    fn new_oop() -> Self
    {
        let header = Header::new(
            "C++ Programming".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
            
        let mut num = 1u16;
        let mut questions = Vec::<OldQuestion>::new();
        let qb = OldQuestion
        {
            question: r#"Which ones of the following are not correct?  …… (   ,   )
            #include <iostream>
            using namespace std;
            int main();
            {    // This is my first program.
                cout << "Hello, World!" << endl
                return 0;
            }"#,
            choice: [r#"int main();"#,
                    r#"{    // This is my first program"#,
                    r#"cout << "Hello, World!" << endl"#,
                    r#"return 0;"#],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;   // num = 2
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language grammar?  …… (   )",
            choice: [";            is the end mark of the C statement.",
                    "/* Comment */        is a multi-line comment.",
                    "// Comment        is a single line comment.",
                    "A ~ Z, a ~ z, _, # and numbers 0 ~ 9 can all be used for variable names."],
            group: num,
            answer: 4,
        };
        questions.push(qb); 

        num += 1;  // num = 3
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language grammar?  …… (   )",
            choice: ["C++ is case-sensitive.",
                    "Whitespaces in C++ language are blank, tab, new line character.",
                    "A multi-line comment starts with the characters */ and terminates with /*",
                    "You cannot have comments within comments and they do not occur within a string or character literals."],
            group: num,
            answer: 0x3,
        };
        questions.push(qb);

        num += 1;  // num = 4
        let qb = OldQuestion
        {
            question: "Which one of the following cannot be a variable in C++ language?  …… (   )",
            choice: ["abc",
                    "0x123",
                    "a_123",
                    "_temp"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 5
        let qb = OldQuestion
        {
            question: "Which one of the following can not you use as a variable name in C++ language?  …… (   )",
            choice: ["if",
                    "int",
                    "for",
                    "until"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 6
        let qb = OldQuestion
        {
            question: "Which one of the following is not the same as below in C++ language?   …… (   )
            int a = 2 + 3;",
            choice: ["inta = 2 + 3;",
                    "int a=2+3;",
                    "int a = 2+3;",
                    "int a=2 + 3;"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 7
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language grammar?  …… (   )",
            choice: ["char is a data type of one byte.",
                    "int is a data type of four bytes.",
                    "long long is a data type of eight bytes.",
                    "float is a data type of eight bytes."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 8
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language grammar?  …… (   )",
            choice: ["char is the same as signed char.",
                    "long is a data type of four bytes under Windows and of eight bytes under Linux.",
                    "unsigned is the same data type as unsigned int.",
                    "unsigned float is used for only positive float numbers."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 9
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct in C++ language grammar?  …… (   ,   )",
            choice: ["Lvalue is changeable, mutable, and inconstant.",
                    "In the statement const int PI = 3.14159;, PI is an example of Rvalue.",
                    "Literal expressions such as 6, 1.414, ‘a’, “Hello” are examples of Lvalue.",
                    "Rvalue can be assigned a value to."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 10
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language?  …… (   )",
            choice: ["0xf == 017 == 0b1111 == 15",
                    "0x19 == 031 == 0b11001 == 25",
                    "0x8 == 08 == 0b1000 == 8",
                    "0x1 == 01 == 0b1 == 1"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 11
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct?  …… (   ,   )",
            choice: ["One byte is eight bits.",
                    "1 KiB == 1000 B",
                    "1 MB == 1000 KB",
                    "1 GiB == 1024 KiB"],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 12
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language grammar?  …… (   )",
            choice: ["Constants are immutable or unchangeable variables.",
                    "Constants can be any of the basic data types such as an integer constant, a floating constant, a character constant and so on.",
                    "Literals are the expression of values such as 1, 3.14, ‘a’, “hello world!” and so on.",
                    "Constants can be called Lvalue in some contexts."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 13
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the correct output of the C++ program below?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int    a = 10, b = 20;
                cout << "a = " << ++a << " and b = " << b++ << endl;
                return 0;
            }"#,
            choice: ["a = 10 and b = 20",
                    "a = 11 and b = 20",
                    "a = 10 and b = 21",
                    "a = 11 and b = 21"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 14
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language?  …… (   )",
            choice: ["(0 && 0) == 0",
                    "(1 && 0) == 1",
                    "(0 && 1) == 0",
                    "(1 && 1) == 1"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 15
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language?  …… (   )",
            choice: ["(0 || 0) == 0",
                    "(1 || 0) == 1",
                    "(0 || 1) == 0",
                    "(1 || 1) == 1"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 16
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the C++ language?  …… (   )",
            choice: ["!(0 && 0) == 0",
                    "!(1 || 0) == 0",
                    "!(0 && 1) == 1",
                    "!(1 || 1) == 0"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 17
        let qb = OldQuestion
        {
            question: "Which one of the following is correct for the C++ language?  …… (   )",
            choice: ["(0b1100 & 0b1010) == 0b1000",
                    "(0b1100 & 0b1010) == 0b1001",
                    "(0b1100 & 0b1010) == 0b0110",
                    "(0b1100 & 0b1010) == 0b1110"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 18
        let qb = OldQuestion
        {
            question: "Which one of the following is correct for the C++ language?  …… (   )",
            choice: ["(0b1100 | 0b1010) == 0b1000",
                    "(0b1100 | 0b1010) == 0b1001",
                    "(0b1100 | 0b1010) == 0b0110",
                    "(0b1100 | 0b1010) == 0b1110"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 19
        let qb = OldQuestion
        {
            question: "Which one of the following is correct for the C++ language?  …… (   )",
            choice: ["(0b1100 ^ 0b1010) == 0b1000",
                    "(0b1100 ^ 0b1010) == 0b1001",
                    "(0b1100 ^ 0b1010) == 0b0110",
                    "(0b1100 ^ 0b1010) == 0b1110"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 20
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct in C++ language?  …… (   ,   )",
            choice: ["~(0b1100 & 0b1010) == 0b1010",
                    "~(0b1100 | 0b1010) == 0b0001",
                    "~(0b1100 ^ 0b1010) == 0b0110",
                    "!(0b1100 & 0b1010) == 0b0000"],
            group: num,
            answer: 0x42
        };
        questions.push(qb);

        num += 1;  // num = 21
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect for the C++ language grammar?  …… (   )",
            choice: ["(0b10010110 << 1) == 0b00101100",
                    "(0b10010110 >> 1) == 0b01001011",
                    "(4 << 1) == 4 * 2",
                    "(4 >> 1) == 4 * 2"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 22
        let qb = OldQuestion
        {
            question: "Which one of the following is the result of the below?  …… (   )
            ((8 >> 2) == (8 / 4)) ? (1 + (2 >> 1)) : (2 - (1 << 1))",
            choice: ["0",
                    "1",
                    "2",
                    "3"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 23
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int a = 1, b = 2;
                a = b;
                b = a;
                cout << "a = " << a << " and b = " << b << endl;
                return 0;
            }"#,
            choice: ["a = 2 and b = 2",
                    "a = 1 and b = 1",
                    "a = 2 and b = 1",
                    "a = 1 and b = 2"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 24
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int a = 1, b = 2;
                int tmp = a;
                a = b;
                b = tmp;
                cout << "a = " << a << " and b = " << b << endl;
                return 0;
            }"#,
            choice: ["a = 2 and b = 2",
                    "a = 2 and b = 1",
                    "a = 1 and b = 2",
                    "a = 1 and b = 1"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 25
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int a = 1, b = 3, c = 2, d = 4;
                int m = (a > b) ? a : b;
                m = (m > c) ? m : c;
                m = (m > d) ? m : d;
                cout << "m = " << m << endl;
                return 0;
            }"#,
            choice: ["m = 1",
                    "m = 2",
                    "m = 3",
                    "m = 4"],
            group: num,
            answer: 4,
        };
        questions.push(qb);


        num += 1;  // num = 26
        let qb = OldQuestion
        {
            question: r#"Which one of the following will be an error?  …… (   )
            class Parent
            {
            public:
                int        ip;
            };
            class Child : public Parent
            {
            public:
                int        ic;
            };
            Parent    p;
            Child    c;"#,
            choice: ["p.ip = 0;",
                    "p.ic = 1;",
                    "c.ip = 2;",
                    "c.ic = 3;"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 27
        let qb = OldQuestion
        {
            question: r#"Which one of the following will be an error?  …… (   ,   )
            class Parent
            {
            public:
                int        ip;
            };
            class Child : public Parent
            {
            public:
                int        ic;
            };
            Child* cp = new Parent;
            Parent* pc = new Child;
            pc->ip = 4;
            pc->ic = 5;"#,
            choice: ["Child* cp = new Parent;",
                    "Parent* pc = new Child;",
                    "pc->ip = 4;",
                    "pc->ic = 5;"],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);

        num += 1;  // num = 28
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            void swap(int a, int b);
            int main()
            {
                int a = 1, b = 2;
                swap(a, b);
                cout << “a = ” << a << “ and b = ” << b << endl;
                return 0;
            }
            void swap(int a, int b)
            {
                int tmp = a;
                a = b;
                b = tmp;
            }"#,
            choice: ["a = 2 and b = 2",
                    "a = 2 and b = 1",
                    "a = 1 and b = 2",
                    "a = 1 and b = 1"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 29
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int a = 1, b = 2;
                int c = (a + b) << 2;
                if (c < 0)
                    cout << “negative!” << endl;
                else if (c == 0)
                    cout << “zero!” << endl;
                else if (c > a + b)
                    cout << “c is 12.” << endl;
                else
                    cout << “c is positive.” << endl;
                return 0;
            }"#,
            choice: ["negative!",
                    "zero!",
                    "c is 12.",
                    "c is positive."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 30
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int a = 1, b = 2;
                int c = (a + b) >> 2;
                switch ©
                {
                case -1:    cout << “negative!” << endl;   	    break;
                case 0:   	cout << “zero!” << endl;   	        break;
                case 6:   	cout << “c is positive.” << endl;  	break;
                default:    cout << “c is 12.” << endl;   		break;
                }
                return 0;
            }"#,
            choice: ["negative!",
                    "zero!",
                    "c is 12.",
                    "c is positive"],
            group: num,
            answer: 0x2,
        };
        questions.push(qb);

        num += 1;  // num = 31
        let qb = OldQuestion
        {
            question: "Which one of the following is not an infinite loop?  …… (   )",
            choice: [
    r#"LABEL1:
        cout << “FOREVER” << endl;
        return;
        goto LABEL1;"#,
    r#"for (;;)
        cout << “FOREVER” << endl;"#,
    r#"while (1)
        cout << “FOREVER” << endl;"#,
    r#"do
        cout << “FOREVER” << endl;;
    while (1);"#],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 32
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int arr[] = { 1, 2, 3, 4, 5 };
                int a = arr[2]--;
                cout << “The answer is ” <<  arr[a] << endl;
                return 0;
            }"#,
            choice: ["The answer is 3.",
                    "The answer is 4.",
                    "The answer is 5.",
                    "The answer is 6."],
            group: num,
            answer: 0x2,
        };
        questions.push(qb);

        num += 1;  // num = 33
        let qb = OldQuestion
        {
            question: r#"Which one of the following is the output of the below program?  …… (   )
            #include <iostream>
            using namespace std;
            int main()
            {
                int arr[] = { 1, 2, 3, 4, 5 };
                int* ptr = arr;
                ptr++;
                cout << “The answer is ” << arr[1] - *ptr << endl;
                return 0;
            }"#,
            choice: ["The answer is 0.",
                    "The answer is 1.",
                    "The answer is 2.",
                    "The answer is 3."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 34
        let qb = OldQuestion
        {
            question: "Which one of the following is wrong about access modifiers?  …… (   )",
            choice: ["Private – default for class; invisible to the outside",
                    "Protected – visible only to derived classes",
                    "Public – default for struct; visible to the outside",
                    "Default – private for struct; public for class"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 35
        let qb = OldQuestion
        {
            question: "Which one of the following is wrong about struct and class?  …… (   )",
            choice: ["Struct is exactly the same as class except that public is default in struct while private is default in class.",
                    "Both class and struct can have bit fields.",
                    "A struct can have only member variable(s) while a class can have both member variable(s) and member function(s).",
                    "Both class and struct can be inherited."],
            group: num,
            answer: 3,
        };
        questions.push(qb);
        Self { header, questions }
    }

    fn new_dsa() -> Self
    {
        let header = Header::new(
            "Data Structure and Algorithms".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
    
        let mut num = 1u16;
        let mut questions = Vec::<OldQuestion>::new();
        let mut qb = OldQuestion
        {
            question: "Which ones of the following are correct?   ……. (   ,   )",
            choice: ["Algorithm is a step-by-step procedure, which defines a set of instructions to be executed in a certain order to get the desired output.",
                    "Algorithms are generally created independent of underlying languages, i.e. an algorithm can be implemented in more than one programming language.",
                    "Algorithm is a random-ordered procedure, which defines a set of instructions to be executed in a random order to get the desired output.",
                    "Algorithms are generally created dependent on underlying languages, i.e. an algorithm can be implemented in only one programming language."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;   // num = 2
        qb = OldQuestion
        {
            question: "Which one of the following is not Characteristics of an Algorithm?  …… (   )",
            choice: ["All procedures can be called an algorithm.",
                    "An algorithm should be clear and unambiguous.",
                    "An algorithm should have 0 or more well-defined inputs.",
                    "An algorithm should have 1 or more well-defined outputs, and should match the desired output."],
            group: num,
            answer: 1,
        };
        questions.push(qb); 

        num += 1;  // num = 3
        let qb = OldQuestion
        {
            question: "Which one of the following is not Characteristics of an Algorithm?  …… (   )",
            choice: ["An algorithm must terminate after a finite number of steps.",
            "An algorithm should be feasible with the available resources.",
            "An algorithm should have random directions, which should be dependent on only one programming code.",
            "Each of its steps (or phases), and their inputs/outputs should be clear and must lead to only one meaning."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 4
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about an Algorithm?   …… (   )",
            choice: ["There are no well-defined standards for writing algorithms.",
            "Algorithms are problem and resource dependent.",
            "Algorithms are never written to support a particular programming code.",
            "We do not have to know the problem domain, for which we are designing a solution."],
            group: num,
            answer: 4 ,
        };
        questions.push(qb);

        num += 1;  // num = 5
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about an Algorithm?   …… (   )",
            choice: ["We design an algorithm to get a solution to a given problem.",
            "A problem can be solved in more than one way.",
            "Hence, many solution algorithms can be derived for a given problem.",
            "The next step is to accept those proposed solution algorithms without any analysis and implement the best suitable solution."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 6
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to algorithm analysis?   …… (   ,   )",
            choice: ["Priori Analysis which is a theoretical analysis of an algorithm.",
                    "Posterior Analysis which is an empirical analysis of an algorithm.",
                    "Numerical Analysis which is a mathematical analysis of an algorithm.",
                    "Domain Analysis which is a study about the domain."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 7
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about an Algorithm analysis?   …… (   )",
            choice: ["Efficiency of an algorithm is measured by assuming that all other factors, for example, processor speed, are constant and have no effect on the implementation.",
                    "The selected algorithm is implemented using a programming language.",
                    "This is then executed on the target computer machine.",
                    "Actual statistics like running time and space required, are not considered."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 8
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct?   …… (   ,   )",
            choice: ["Algorithm analysis deals with the actual development or coding time of various operations involved.",
                    "Algorithm analysis deals with the execution or running time of various operations involved.",
                    "The running time of an operation can be defined as the number of computer instructions executed per operation.",
                    "The coding time of an operation can be defined as the number of computer instructions executed per operation."],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 9
        let qb = OldQuestion
        {
            question: "Which ones of the following are the two main factors of the algorithm, which decide the efficiency?   …… (   ,   )",
            choice: ["Loop Factor − Loop is measured by counting the number of key loops required by the algorithm.",
                    "Variable Factor –  Variable is measured by counting the number of variables required by the algorithm.",
                    "Time Factor − Time is measured by counting the number of key operations such as comparisons in the sorting algorithm.",
                    "Space Factor − Space is measured by counting the maximum memory space required by the algorithm."],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 10
        let qb = OldQuestion
        {
            question: "Which one of the following is correct?   …… (   )",
            choice: ["The complexity of an algorithm f(n) gives the running time and/or the storage space required by the algorithm in terms of n as the size of input data.",
                    "The complexity of an algorithm f(n) gives the coding time and/or the storage space required by the algorithm in terms of n as the size of input data.",
                    "The complexity of an algorithm f(n) gives the running time and/or the variables required by the algorithm in terms of n as the size of input data.",
                    "The complexity of an algorithm f(n) gives the coding time and/or the variables required by the algorithm in terms of n as the size of input data."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 11
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about space complexity?  …… (   )",
            choice: ["Space complexity of an algorithm represents the amount of memory space required by the algorithm in its life cycle.",
                    "The space required by an algorithm is equal to the sum of a fixed part and a variable part.",
                    "A fixed part of the space is a space required to store certain data and variables, and vary according to the size of the problem. For example, simple variables and constants used, program size, etc.",
                    "A variable part of the space is a space required by variables, whose size depends on the size of the problem. For example, dynamic memory allocation, recursion stack space, etc."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 12
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about time complexity?  …… (   )",
            choice: ["Time complexity of an algorithm represents the amount of time required by the algorithm to prepare for computation.",
                    "Time complexity of an algorithm represents the amount of time required by the algorithm to run to completion.",
                    "Time requirements can be defined as a numerical function T(n), where T(n) can be measured as the number of steps, provided each step consumes constant time.",
                    "If the total computational time is T(n) = c ∗ n for addition of two n-bit integers takes n steps, where c is the time taken for the addition of two bits, we observe that T(n) grows linearly as the input size increases."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 15
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the execution time cases?   …… (   )",
            choice: ["Best Case − Minimum time required for program execution.",
                    "Normal Case – Randomly measured time required for program execution.",
                    "Average Case − Average time required for program execution.",
                    "Worst Case − Maximum time required for program execution."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 16
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the asymptotic notation?   …… (   )",
            choice: ["Ο Notation - Worst case",
                    "Ω Notation - Best case",
                    "β Notation - Sort of Random case",
                    "θ Notation - sort of Average case"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 17
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about big O notation?   …… (   )",
            choice: ["The notation Ο(n) is the formal way to express the upper bound of an algorithm's running time.",
                    "The notation Ο(n) is not usually accepted by the software industry because it is about the worst case.",
                    "It measures the worst case time complexity or the longest amount of time that an algorithm can possibly take to complete.",
                    "Mathematically, if f(n) describes the running time of an algorithm; f(n) is O(g(n)) if there exist positive constant C and n0 such that, 0 <= f(n) <= C·g(n) for all n >= n0."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 20
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about greedy algorithms?   …… (   )",
            choice: ["The greedy algorithm is designed to achieve optimum solution for a given problem.",
                    "The greedy algorithm always leads to the optimized solutions.",
                    "In a greedy algorithm approach, decisions are made from the given solution domain.",
                    "As being greedy, the closest solution that seems to provide an optimum solution is chosen."],
            group: num,
            answer: 2
        };
        questions.push(qb);

        num += 1;  // num = 21
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about greedy algorithms?   …… (   )",
            choice: ["Greedy algorithms always fail to get optimized solutions.",
                    "Greedy algorithms try to find a localized optimum solution, which may eventually lead to globally optimized solutions.",
                    "However, in general, greedy algorithms do not provide globally optimized solutions.",
                    "Hence, we may conclude that the greedy approach picks an immediate optimized solution and may fail where global optimization is a major concern."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 22
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the divide-and-conquer approach?   …… (   )",
            choice: ["In the divide and conquer approach, the problem in hand is divided into smaller subproblems and then each problem is solved independently.",
                    "When we keep on dividing the subproblems into even smaller subproblems, we may eventually reach a stage where no more division is possible.",
                    "Those 'atomic' smallest possible subproblems (fractions) are solved.",
                    "Sometimes, the solutions of all subproblems cannot be finally merged so that the solution to an original problem cannot be obtained."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 23
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the divide-and-conquer approach?  …… (   )",
            choice: ["Broadly, we can understand the divide-and-conquer approach in a three-step process.",
                    "The divide step of the divide and conquer approach involves breaking the problem into smaller subproblems.",
                    "Subproblems should represent a part of the original problem.",
                    "The divide step generally takes a repetitive approach to divide the problem until no sub-problem is further divisible."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 24
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the divide-and-conquer approach?  …… (   )",
            choice: ["At the divide stage, subproblems become atomic in nature but still represent some part of the actual problem.",
                    "The conquer step receives a lot of smaller subproblems to be solved.",
                    "Generally, at the conquer level, the problems are considered 'unsolved' on their own.",
                    "When the smaller subproblems are solved, the merge stage recursively combines them until they formulate a solution to the original problem."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 25
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about the divide-and-conquer approach?  …… (   )",
            choice: ["This algorithmic approach works recursively and conquer & merge steps work so close that they appear as one.",
                    "This algorithmic approach works recursively and conquer & merge steps work so independently that they clearly appear as two steps.",
                    "This algorithmic approach works repetitively and conquer & merge steps work so close that they appear as one.",
                    "This algorithmic approach works repetitively and conquer & merge steps work so independently that they clearly appear as two steps."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 27
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the dynamic programming approach?  …… (   )",
            choice: ["Rather, the results of these smaller subproblems are remembered and used for similar or overlapping subproblems.",
                    "Dynamic programming is used where we have problems, which can be divided into similar subproblems, so that their results can be reused.",
                    "All the divide and conquer algorithms can be transferred into dynamic programming algorithms.",
                    "Mostly, these algorithms are used for optimization."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 28
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the dynamic programming approach?  …… (   )",
            choice: ["Before solving the in-hand subproblem, a dynamic algorithm will try to examine the results of the previously solved subproblems.",
                    "The problem should be able to be divided into smaller independent subproblems.",
                    "The solutions of subproblems are combined in order to achieve the best solution.",
                    "An optimum solution can be achieved by using an optimum solution of smaller subproblems."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 29
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the dynamic programming approaches?  …… (   )",
            choice: ["Dynamic programming should be used only in top-down manner.",
                    "In contrast to greedy algorithms, where local optimization is addressed, dynamic algorithms are motivated for an overall optimization of the problem.",
                    "In contrast to divide and conquer algorithms, where solutions are combined to achieve an overall solution, dynamic algorithms use the output of a smaller subproblem and then try to optimize a bigger subproblem.",
                    "Dynamic algorithms use Memoization to remember the output of already solved subproblems."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 30
        let qb = OldQuestion
        {
            question: "Data Definition defines a particular data with the following characteristics. Which one of the following is not correct about the data definition?  …… (   )",
            choice: ["Atomic − Definition should define a single concept.",
                    "Traceable − Definition should be able to be mapped to some data element.",
                    "Accurate − Definition should be ambiguous.",
                    "Clear and Concise − Definition should be understandable."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 32
        let qb = OldQuestion
        {
            question: "There are two data types: Built-in Data Type and Derived Data Type. Which one of the following is not correct about the data type?  …… (   )",
            choice: ["Those data types for which a language has built-in support are known as Built-in Data types.",
                    "All the languages provide the built-in data types.",
                    "Built-in data types provided by a programming language vary from one programming language to another.",
                    "C language provides Integers, Boolean (true, false), Floating (Decimal numbers), Character and Strings as Built-in data types."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 33
        let qb = OldQuestion
        {
            question: "There are two data types: Built-in Data Type and Derived Data Type. Which one of the following is not correct about the data type?  …… (   )",
            choice: ["The derived data types which are implementation independent as they can be implemented in one or the other way are known as derived data types.",
                    "The derived data types are normally built by the combination of primary or built-in data types and associated operations on them.",
                    "List, Array, Stack and Queue are all the examples of the derived data types.",
                    "The derived data types are made only by combination of built-in data types."],
            group: num,
            answer: 4,
        };
        questions.push(qb);
        Self { header, questions }
    }

    pub fn new_prob() -> Self
    {
        let header = Header::new(
            "Probabilty Theory".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
        
        let mut questions = Vec::<OldQuestion>::new();

        let mut num = 1u16;
        let qb = OldQuestion
        {
            question: "If a coin was tossed 1000 times, heads came up 530 times and tails 470 times. Does this coin come up heads more often? ….… (   )",
            choice: ["On a fair coin, the probability of getting 530 heads is 3.1%. So, it is judged that the coin comes up heads more often.",
                    "On a fair coin, the probability of getting 530 heads is 3.1%. So, it is judged that the coin comes up tails more often.",
                    "On a fair coin, the probability of getting 530 heads is 3.1%. But, we cannot say that the coin comes up heads more often.",
                    "On a fair coin, the probability of getting 530 heads is 3.1%. But, we can say that the coin is a fair coin."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;   // num = 2
        let qb = OldQuestion
        {
            question: "In the presidential election in which A and B were nominated, a poll of 1,000 people showed that 530 and 470 people supported A and B, respectively. Is candidate A currently in a better situation?  ….… (   )",
            choice: ["In a fair condition, the probability of getting 530 for candidate A is 3.1%. So, it is judged that candidate A is in a better situation.",
                    "In a fair condition, the probability of getting 530 for candidate A is 3.1%. So, it is judged that candidate B is in a better situation.",
                    "In a fair condition, the probability of getting 530 for candidate A is 3.1%. But, we cannot say that candidate A is in a better situation.",
                    "In a fair condition, the probability of getting 530 for candidate A is 3.1%. But, we can say that none of candidates A and B are in a better situation."],
            group: num,
            answer: 1,
        };
        questions.push(qb); 

        num += 1;   // num = 3
        let qb = OldQuestion
        {
            question: "In option 1, you will surely get 5,000 soms. In option 2, you will get 10,000 soms. Which option is better?  ….… (   )",
            choice: ["Option 1 and option 2 are the same because their expected values are both 5,000 soms.",
                    "Option 1 is better even though their expected values are both 5,000 soms.",
                    "Option 2 is better even though their expected values are both 5,000 soms.",
                    "None of option 1 and option 2 are better because their expected values are both 5,000 soms."],
            group: num,
            answer: 2,
        };
        questions.push(qb); 

        num += 1;  // num = 4
        let qb = OldQuestion
        {
            question: "You want to play roulette with 1 million soms. The goal is to make 2 million soms without bankruptcy. How much are you going to bet each time?  ….… (   )",
            choice: ["10,000 soms",
                    "100,000 soms",
                    "500,000 soms",
                    "1,000,000 soms"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 5
        let qb = OldQuestion
        {
            question: "You are in a rock-paper-scissors competition. Your opponent has a strategy that he will show rock at 50%, paper at 30% and scissor at 20%. Which strategy are you going to take in order to guarantee to win?  ….… (   )",
            choice: ["Give rock at 100%",
                    "Give scissor at 100%",
                    "Give paper at 100%",
                    "Give rock at 33.3%, paper at 33.3% and paper at 33.3%"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 6
        let qb = OldQuestion
        {
            question: "Which one of the following is the best strategy for rock-paper-scissors competition?  ….… (   )",
            choice: ["Give rock at 100%",
                    "Give scissor at 100%",
                    "Give paper at 100%",
                    "Give rock at 33.3%, paper at 33.3% and paper at 33.3%"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 7
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about fair dice throwing?  ….… (   )",
            choice: ["Probability of getting 3 in one throw is 1/6.",
                    "Probability of getting a sum of 6 when thrown twice is 5/36.",
                    "Probability of getting 7 in one throw is 1/6.",
                    "Probability of getting a sum of 7 when thrown twice is 6/36."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 8
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about the law of large numbers?  ….… (   )",
            choice: ["The mean of the data approaches the theoretical expected value.",
                    "A universal principle does not have anything to do with the law of large numbers.",
                    "A universal principle that holds regardless of the type of data",
                    "By averaging the data through the universal principle of the law of large numbers, relatively accurate estimates of expected values are possible."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 9
        let qb = OldQuestion
        {
            question: "You performed a survey to ask whether the participants have a secret that they have never told anybody. In the survey, participants toss an honest coin. They answer honestly if the coin’s outcome is Head while they write a random answer ‘yes/no’ with a probability 50% if the coin’s outcome is Tail. Then, If the proportion of ‘yes’ responses is 30%, what is the proportion of those who have a secret that they have never told anybody?  ….… (   )",
            choice: ["about 5%",
                    "about 10%",
                    "about 15%",
                    "about 20%"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 10
        let qb = OldQuestion
        {
            question: "One envelope has 10 times more money than the other. You can choose one of them to take the money inside. You chose the envelope A. You are given a chance to change your choice. Will you change your choice?  ….… (   )",
            choice: ["I have to change my choice because envelope B has ten times more money than A.",
                    "I should not change my choice because envelope A has ten times more money than B.",
                    "I don’t have to change my choice because there is no reason to change.",
                    "I should not change my choice because I accepted my fate."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 11
        let qb = OldQuestion
        {
            question: "If you throw a die twice and you are interested in the sum of the spots of the two dice, what can be defined to be a random variable?  ….… (   )",
            choice: ["The sum of the outcome spots of two dice",
                    "The outcome spots of each dice",
                    "The situation where a die will be thrown twice",
                    "All possible outcome spots of each dice"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 12
        let qb = OldQuestion
        {
            question: "Who is considered to be the designer of probability theory?  ….… (   )",
            choice: ["Albert Einstein",
                    "Александр Сергеевич Пушкин",
                    "Carl Friedrich Gauss",
                    "Андре́й Никола́евич Колмого́ров"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 13
        let qb = OldQuestion
        {
            question: "If you throw a dice twice and you are interested in the sum of outcome spots of the two dice, what will be the sample space?  ….… (   )",
            choice: ["{ 1, 2, 3, 4, 5, 6 }",
                    "{ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 }",
                    "{ 0, 1, 2, 3, 4, 5 }",
                    "{ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 }"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 14
        let qb = OldQuestion
        {
            question: "Two teams, A and B, will play a best-of-five game and the winning team will receive 16,000 soms as a prize. The game is stopped after A has won the first game. How should the prize money be divided reasonably?  ….… (   )",
            choice: ["Team A will receive 11,000 soms while team B will receive 5,000 soms.",
                    "Team A will receive 10,000 soms while team B will receive 6,000 soms.",
                    "Team A will receive 9,000 soms while team B will receive 7,000 soms.",
                    "Team A will receive 8,000 soms while team B will receive 8,000 soms."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 15
        let qb = OldQuestion
        {
            question: "When we define probability through a sample space, what should we take into account?   ….… (   ,   )",
            choice: ["Most of the possible outcomes",
                    "All the thinkable outcomes including impossible outcomes",
                    "All the possible outcomes",
                    "The probability of each outcome"],
            group: num,
            answer: 0x34,
        };
        questions.push(qb);

        num += 1;  // num = 16
        let qb = OldQuestion
        {
            question: "One envelope has 10 times more money than the other. You can choose one of them to take the money inside. Which one of the following is the correct sample space?  ….… (   )",
            choice: ["{ (x, 10x), (x, 0.1x) } where x is the amount of money in envelope A",
                    "{ (10x, x), (0.1x, x) } where x is the amount of money in envelope B",
                    "{ (0.1x, 10x), (x, x) } where x is the amount of money in the envelope that has less money",
                    "{ (x, 10x), (10x, x) } where x is the amount of money in the envelope that has less money"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 17
        let qb = OldQuestion
        {
            question: "Mr. A has two children. Which one of the following is correct?  ….… (   )",
            choice: ["The probability that both of them are daughters is 1/3.",
                    "If one of them is a daughter. the probability that both of them are daughters is 1/2.",
                    "If the first one of them is a daughter,the probability that both of them are daughters is 1.",
                    "If one of them is a daughter and she was born on Monday, the probability that both of them are daughters is 13/27."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 18
        let qb = OldQuestion
        {
            question: "There is a prize behind only one of the three doors. If there is a prize behind the selected door, you can claim it. The host opens the empty door among the remaining two doors and gives you the opportunity to change the choice. Will you change your choice as information is added and why?  ….… (   )",
            choice: ["Yes, I will change my choice because the probability that there is a price behind the other door is 2/3.",
                    "Yes, I will change my choice because the probability that there is a price behind the other door is 1.",
                    "No, I will keep my choice because the probability that there is a price behind my chosen door is 2/3.",
                    "No, I will keep my choice because the probability that there is a price behind my chosen door is 1."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 19
        let qb = OldQuestion
        {
            question: "Which one of the following pieces of advice is correct?  ….… (   ,   )",
            choice: ["Acquisition of information changes probability.",
                    "Infer probabilities by actively reflecting information.",
                    "Acquisition of information always raises probability.",
                    "Infer probabilities by actively ignoring additional information."],
            group: num,
            answer: 0x12,
        };
        questions.push(qb);

        num += 1;  // num = 20
        let qb = OldQuestion
        {
            question: "Which one of the following probability is meaningless?  ….… (   )",
            choice: ["Probability that it is snowing exactly at 9 o’clock in the morning tomorrow",
                    "Probability that it is not raining exactly at 9 o’clock in the morning tomorrow",
                    "Probability that the maximum temperature in Bishkek tomorrow will be exactly 27.9 degrees",
                    "Probability that the maximum temperature in Bishkek tomorrow will be 27.9 degrees or above"],
            group: num,
            answer: 3
        };
        questions.push(qb);
        Self { header, questions }
    }

    pub fn new_se() -> Self
    {
        let header = Header::new(
            "Software Engineering".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
        
        let mut questions = Vec::<OldQuestion>::new();
        let mut num = 1u16;
        let qb = OldQuestion
        {
            question: "Which one of the following is correct?   ……. (   )",
            choice: ["Program is an ordered set of instructions written to solve one or more specific problems.",
                    "Software is a synonym of program.",
                    "System is a group of programs.",
                    "One big program can be called a system."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;   // num = 2
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect?   ……. (   )",
            choice: ["Software is developed, but rather engineered in the context of “Software Engineering.”",
                    "Software does not wear out while hardware does.",
                    "However, software deteriorates.",
                    "Its design gets better whenever it is changed. As a result, the software evolutes."],
            group: num,
            answer: 4,
        };
        questions.push(qb); 

        num += 1;  // num = 3
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct?   ……. (   ,   )",
            choice: ["You’d better develop software from scratch.",
                    "You’d better give up enhancing both Quality and Productivity because they have a trade-off relationship.",
                    "You have to consider maintenance when you develop.",
                    "Maintenance cost occupies about ⅔ (two thirds) of the overall software cost"],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 4
        let qb = OldQuestion
        {
            question: "Which one of the following is the advantage of the waterfall model?   ……. (   )",
            choice: ["Easy to know or decide whether a phase is complete or not.",
                    "Best suited to large projects with fixed requirements developed by multiple teams that are geographically remote apart.",
                    "Many mechanisms are explicitly provided for changes in the middle of a process.",
                    "Users do not have to wait until all processes are done to use the software."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 5
        let qb = OldQuestion
        {
            question: "Why is the iterative model used? ……. (   ,   )",
            choice: ["Because the iterative model is best suited to small to medium-sized software development.",
                    "Because user’s feedback may be considered effectively for the next version if you use the iterative model.",
                    "Because linear processes will not repeat to release new versions at each loop if you use the iterative model.",
                    "Because predetermined staffing is possible: you can start with a certain group of engineers and keep the size of the staff constant to the end."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 6
        let qb = OldQuestion
        {
            question: "Which ones of the following are the characteristics of the spiral model? ……. (   ,   )",
            choice: ["It is a risk-driven process model.",
                    "It is easy to convince customers that the iteration is controllable.",
                    "It is good for changes required in the middle of development activities.",
                    "It is best suited to a middle-scale system development."],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 7
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the prototyping model? ……. (   )",
            choice: ["Its prototype is not a working system but shows the functional and non-functional figures of the final product.",
                    "It is good for either a stand-alone process model for small to medium-sized projects or, more often, a technique within other models of any size.",
                    "Final product is visible to the customer at the end of the development.",
                    "It is the best communication tool between customers (stakeholders) and engineers."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 8
        let qb = OldQuestion
        {
            question: "Which model of the following does NOT belong to an iterative model? ……. (   )",
            choice: ["Incremental model",
                    "Spiral model",
                    "V-model",
                    "Exploratory model"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 9
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the component-based model? ……. (   )",
            choice: ["It enables software reuse.",
                    "Cost reduction can be expected.",
                    "Development cycle can be reduced.",
                    "The number of components can be reduced."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 10
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the formal model? ……. (   )",
            choice: ["Specification takes less time, but takes longer time for design and implementation.",
                    "Specification can be mathematically proved.",
                    "It enables flawless software development.",
                    "Programs are generated automatically by transformation of specification to code."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 11
        let qb = OldQuestion
        {
            question: "Why should RSD (Rapid Software Development) be considered today? ……. (   ,   )",
            choice: ["Because software must be developed carefully.",
                    "Because business environments change rapidly.",
                    "Businesses cannot tolerate lower quality software.",
                    "Because businesses must come up with new opportunities and competition."],
            group: num,
            answer: 0x42,
        };
        questions.push(qb);

        num += 1;  // num = 12
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT valued in the Agile model? ... (   )",
            choice: ["Individuals and interactions over Processes and tools",
                    "Comprehensive documentation over Working software",
                    "Customer collaboration over Contract negotiation",
                    "Responding to change over Following a plan"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 13
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT what Agile manifesto is based on? ……. (   )",
            choice: ["Customer satisfaction by rapid delivery of useful software",
                    "Welcome changing requirements, even late in development",
                    "Working software is delivered frequently (weeks rather than months).",
                    "Bugless software is the principal measure of progress."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 14
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT what Agile manifesto is based on? ……. (   )",
            choice: ["Sustainable development, able to maintain a constant pace",
                    "Close, daily cooperation between business people and developers",
                    "Written conversation is the best form of communication.",
                    "Projects are built around motivated individuals, who should be trusted."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 15
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT what Agile manifesto is based on? ……. (   )",
            choice: ["Continuous attention to technical excellence and good design",
                    "Simplicity — the art of maximizing the amount of work done — is essential",
                    "Self-organizing teams",
                    "Regular adaptation to changing circumstances"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 16
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the XP (eXtreme Programming) planning of the Agile model proposed by Kent Beck? ……. (   )",
            choice: ["“User stories” are created.",
                    "Stories are grouped as a deliverable increment.",
                    "After the first increment, “project velocity” is analyzed.",
                    "Schedule for remaining increments is defined by the amount of the work."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 17
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the XP (eXtreme Programming) design of the Agile model proposed by Kent Beck? ……. (   )",
            choice: ["Use of CRC (Class-Responsibility-Collaborator) cards",
                    "For difficult design problems “spike solutions” are tried.",
                    "Prototyping is conducted for some spike solutions, but not always.",
                    "“Refactoring” is not required."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 18
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the XP (eXtreme Programming) coding of the Agile model proposed by Kent Beck? ……. (   )",
            choice: ["Refactoring is always done after coding.",
                    "Unit tests for a story are generated before coding.",
                    "Reduces errors and enhances quality.",
                    "“Pair programming” – does not lower productivity."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 19
        let qb = OldQuestion
        {
            question: "Which ones of the following are the characteristics of the XP (eXtreme Programming) testing of the Agile model proposed by Kent Beck? ……. (   ,   )",
            choice: ["Unit tests are executed daily.",
                    "Overall tests are executed regularly.",
                    "“Fault tolerance tests” are essential.",
                    "“Acceptance tests” are defined by the customer."],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);

        num += 1;  // num = 20
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the characteristic of the Scrum of the Agile model proposed by Schwaber and Beedle? ……. (   )",
            choice: ["Software is partitioned into “packets.”",
                    "Test the product, and then document the product in series.",
                    "“Sprints” are work tasks derived from process patterns.",
                    "Each process pattern is proven to be effective to the project with a tight timeline, changing requirements, and criticality of business."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 21
        let qb = OldQuestion
        {
            question: "Which one of the following is the characteristic of the Scrum of the Agile model proposed by Schwaber and Beedle? ……. (   )",
            choice: ["A sprint is derived from a “backlog” of future requirements.",
                    "15 minutes meetings are conducted every evening.",
                    "“Demos” are delivered to the customer according to the time-box.",
                    "New functionality is demonstrated at the beginning of the sprint."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 22
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT the question that the Scrum team members respond to at the Scrum meeting of the Agile model proposed by Schwaber and Beedle? ……. (   )",
            choice: ["What have you done since the last Scrum meeting?",
                    "Do you have any obstacles?",
                    "Did you finish the current task?",
                    "What will you do before the next Scrum meeting?"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 23
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to the seven principles of the LSD (Lean Software Development) of the Agile model? ……. (   )",
            choice: ["Eliminate waste",
                    "Amplify learning",
                    "Refactor as much as possible",
                    "Decide as early as possible"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 24
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to the seven principles of the LSD (Lean Software Development) of the Agile model? ……. (   )",
            choice: ["Deliver as fast as possible",
                    "Empower the team",
                    "Build integrity in",
                    "See the detail"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 25
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to the main activities of planning? ……. (   )",
            choice: ["Cost calculation",
                    "Risk Analysis",
                    "Estimation",
                    "Scheduling"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 26
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to Boehm’s Top-Ten List of Software Risk Items? ……. (   )",
            choice: ["Personnel shortfalls",
                    "Unrealistic schedules and budgets",
                    "Developing the wrong software functions",
                    "Ambition to develop too good software"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 27
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to Boehm’s Top-Ten List of Software Risk Items? ……. (   )",
            choice: ["Developing the wrong user interface",
                    "Focussing on non-functional requirements",
                    "Gold plating: adding features that are only marginally useful",
                    "Continuing stream of requirement changes"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 28
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT belong to Boehm’s Top-Ten List of Software Risk Items? ……. (   )",
            choice: ["Shortfalls in externally furnished components",
                    "Shortfalls in externally performed tasks",
                    "Real-time performance shortfalls",
                    "Pursuing to modern computer-science capabilities"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 29
        let qb = OldQuestion
        {
            question: "Which one of the following is NOT proper risk management? …. (   )",
            choice: ["Risk should be avoided if possible.",
                    "Risk should be actively accepted if impossible.",
                    "Risk should be minimized.",
                    "Risk should be exaggerated."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 30
        let qb = OldQuestion
        {
            question: "Why do we have to estimate when we plan to develop software? ……. (   )",
            choice: ["Without an estimation, plans must rely on confidence, and that will lead the project to instability.",
                    "Without an estimation, plans must rely on intuition, and that will lead the project to uncertainty.",
                    "In making a good plan, the estimation is the best information to rely on.",
                    "In making a good plan, the estimation always guarantees that the project will be successful."],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 31
        let qb = OldQuestion
        {
            question: "Which one of the following does not have to be estimated in software development planning? ……. (   )",
            choice: ["Number of the potential customers",
                    "Resources",
                    "Cost",
                    "Schedule"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 32
        let qb = OldQuestion
        {
            question: "Which ones of the following are mainly used to estimate in software development planning? ……. (   ,   )",
            choice: ["Experience",
                    "Computer",
                    "Historical information – quantitative information",
                    "Intuition"],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 33
        let qb = OldQuestion
        {
            question: "Which one of the following efforts does not project estimation require? ……. (   )",
            choice: ["Understanding the project scope",
                    "Seeing the project as a whole",
                    "Reference to the historical information – metrics",
                    "Use of alternative methods of estimation to compare"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 34
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the estimation technique? ……. (   )",
            choice: ["Size-oriented estimation",
                    "Function-oriented estimation",
                    "Object-oriented estimation",
                    "Empirical models"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 35
        let qb = OldQuestion
        {
            question: "Why are function points preferable to LOCs in software development planning? …… (   )",
            choice: ["Function points are independent from programming language.",
                    "LOC can be estimated early in the process.",
                    "Larger LOC may be a sign for a smarter implementation, but may get less points.",
                    "Function points are harder to estimate the impact of reusable components."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 36
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to software development scheduling? …… (   )",
            choice: ["To identify activities from software requirements",
                    "To identify activities dependencies",
                    "To estimate resources for each activities",
                    "To allocate resources to activities"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 37
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about software development scheduling? ……. (   )",
            choice: ["Duration table is usually merged with dependency table",
                    "Dependency table shows what activity depends on what resource.",
                    "Activity network shows the amount of work for each staff member.",
                    "Bar chart shows what activity is allocated to whom."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 38
        let qb = OldQuestion
        {
            question: "Which one of the following does NOT describe a critical path in software development planning? ……. (   )",
            choice: ["Longest path in the Activity Network",
                    "Maximum time required to finish the project",
                    "If any task on the Critical Path is delayed, the overall project schedule is delayed.",
                    "Critical Path must change if any tasks on other paths are delayed."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 39
        let qb = OldQuestion
        {
            question: "According to Kitchenham, Pfleeger, and Garvin's 5 perspectives, which ones are correct about software quality? ……. (   ,   )",
            choice: ["Even if software fails to conform to the original specification, it is quality software.",
                    "Quality is immediately recognized, but cannot be explicitly defined.",
                    "If software meets the end-user's goals, it is quality software.",
                    "The quality of software cannot be measured by inherent characteristics (e.g., functions and features) of the software."],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 40
        let qb = OldQuestion
        {
            question: "When you correct errors in software, after which one of the following will it cost the most? ……. (   )",
            choice: ["Software designing",
                    "Software coding",
                    "Software testing",
                    "Software installation"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 41
        let qb = OldQuestion
        {
            question: "Which one of the following will it cost the most? ……. (   )",
            choice: ["Requirement engineering",
                    "Software implementation",
                    "Software testing",
                    "Software maintenance"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 42
        let qb = OldQuestion
        {
            question: "Which one of the following belongs to the requirements? …… (   ,   )",
            choice: ["The description of services provided by the system",
                    "The description of constraints under which the system operates.",
                    "The description of benefits that users want to get from the system.",
                    "The description of goals why users want to buy the system."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 43
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the categories of the requirements? ……. (   )",
            choice: ["Functional requirements: services to be provided",
                    "Non-functional requirements: constraints imposed",
                    "Government requirements: required by the government",
                    "Domain requirements: required by the application domain"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 44
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the requirements types? ……. (   ,   )",
            choice: ["User requirements: written for or by the customer",
                    "Stakeholder requirements: written for or by the stakeholder",
                    "System requirements: detailed descriptions of functions to be provided and constraints imposed",
                    "Architecture requirements: detailed descriptions of architectures to be provided and limitations imposed"],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 45
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about requirements engineering? ……. (   )",
            choice: ["Requirements engineering is the process of defining, documenting, and maintaining requirements required by all stakeholders unsystematically.",
                    "Requirements engineering is focused on WHATs instead of HOWs of the system under development.",
                    "Requirements engineering should keep being done all the time from the beginning to the end of the project.",
                    "Requirements engineering is needed only in the large-sized projects."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 46
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about requirements? …. (   )",
            choice: ["Requirements should be abstract.",
                    "Requirements should be complete.",
                    "Requirements should be consistent.",
                    "Requirements should be unambiguous."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 47
        let qb = OldQuestion
        {
            question: "Why are requirements analyzed? ……. (   )",
            choice: ["After the system construction is finished, we must check what the customer wants precisely and completely, in terms of required functions and non-functions.",
                    "Before the system test is performed, we must know what the customer wants precisely and completely, in terms of required functions and non-functions.",
                    "Before the system construction starts, we must know what the customer wants precisely and completely, in terms of required functions and non-functions.",
                    "While the system is maintained, we must know what the customer wants precisely and completely, in terms of required functions and non-functions."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 48
        let qb = OldQuestion
        {
            question: "What is the work product of requirements engineering? ……. (   )",
            choice: ["Requirements specification (documents)",
                    "Requirements questionnaire (documents)",
                    "Software manual (documents)",
                    "Software contract (documents)"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 49
        let qb = OldQuestion
        {
            question: "When we perform a feasible study of software, which of the following don’t we have to ask? ……. (   )",
            choice: ["Is the system technically feasible?",
                    "Is the system economically feasible?",
                    "Is the system occasionally feasible?",
                    "Is the system in accordance with organizational objectives?"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 50
        let qb = OldQuestion
        {
            question: "When we perform a feasible study of software, which ones of the following do we need to ask? ……. (   ,   )",
            choice: ["Are there alternative, possibly better solutions?",
                    "Is the system legally feasible?",
                    "Are there cheaper, possibly better solutions?",
                    "Is the system legally feasible?"],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 51
        let qb = OldQuestion
        {
            question: "What should we do in the requirements engineering phase in order to know whether the system is technically feasible? ……. (   ,   )",
            choice: ["To analyze whether user requirements (or needs) can be satisfied with the technology and resources to be found in the near future.",
                    "To analyze whether user requirements (or needs) can be satisfied with the current technology and resources.",
                    "To ask whether the system can be easily replaced with other systems or make the same performance as the replaceable systems.",
                    "To ask whether the system may be integrated with other systems or interact with them."],
            group: num,
            answer: 0x42,
        };
        questions.push(qb);

        num += 1;  // num = 52
        let qb = OldQuestion
        {
            question: "What don’t we have to do at the requirements engineering phase in order to know whether the system is economically feasible? ……. (   )",
            choice: ["To analyze whether the system is cheaper than any other software.",
                    "To analyze whether the system is cost-effective from the user’s point of view.",
                    "To analyze whether the system is of benefit to users.",
                    "To analyze whether the system can be developed within the budget."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 53
        let qb = OldQuestion
        {
            question: "What don’t we have to elicit from customers (stakeholders) when we communicate with them for requirements elicitation? ……. (   )",
            choice: ["Services to be provided",
                    "Constraints to be imposed",
                    "Satisfaction to be expected",
                    "Requirements changes"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 54
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the ways to elicit the requirements? ……. (   )",
            choice: ["Interviews",
                    "Observations: existing system and stakeholders",
                    "Surveys (using questionnaires)",
                    "Guessing"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 55
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the ways to elicit the requirements? ……. (   ,   )",
            choice: ["Guessing",
                    "Scenarios",
                    "Use-cases",
                    "Analyzing other similar software"],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 56
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the ways to identify the elicited requirements? ……. (   )",
            choice: ["Integrate information elicited from stakeholders and existing systems, or business processes.",
                    "Classify and organize the information.",
                    "Prioritize and negotiate requirements.",
                    "Document requirements."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 57
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the activities of requirements analysis? ……. (   )",
            choice: ["Specify operational characteristics",
                    "Specify interface with other systems",
                    "Specify constraints",
                    "Specify how to use"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 58
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the activities of requirements analysis? ……. (   ,   )",
            choice: ["Negotiate contracts",
                    "Contracts validation",
                    "Negotiate requirements",
                    "Requirements validation"],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 59
        let qb = OldQuestion
        {
            question: "Which one of the following belongs to the objectives of requirements analysis? ……. (   )",
            choice: ["To get more confident emotionally",
                    "To refine requirements into tasks",
                    "To find the current problem better",
                    "To communicate with stakeholder better"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 60
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the functional model of scenario-based modeling of requirements modeling? ……. (   )",
            choice: ["It should include non-functional requirements too.",
                    "It is description of software functions",
                    "It should be organized and systematic.",
                    "It may use a structure chart to represent the logical structure of functionalities."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 61
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the use-case model of scenario-based modeling of requirements modeling? ……. (   )",
            choice: ["It is derived from scenarios.",
                    "It identifies interactions between actors and other actors.",
                    "All possible interactions are described.",
                    "For detailed use-cases, Sequence Diagrams may be created to show sequence of events."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 62
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the scenario-based modeling of requirements modeling? ……. (   )",
            choice: ["Scenarios describe the usage of the system by users.",
                    "Use-case describes usage of a system.",
                    "Actors represent roles of users and/or interacting systems in terms of the system functions.",
                    "Actors cannot play different roles."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 63
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the class diagram of the class-based modeling of requirements modeling? ……. (   )",
            choice: ["It defines classes and its roles in detail.",
                    "A class has a class name.",
                    "A class has attributes or data members.",
                    "A class has methods or services."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 64
        let qb = OldQuestion
        {
            question: "Which one of the following is not a synonym of “methods” of a class? ……. (   )",
            choice: ["Operations",
                    "Manipulations",
                    "Member functions",
                    "Attributes"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 65
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["Instance level relationships",
                    "Class level relationships",
                    "User level relationships",
                    "General relationships"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 66
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the instance level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["Association",
                    "Cooperation",
                    "Aggregation",
                    "Composition"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 67
        let qb = OldQuestion
        {
            question: "Which one of the following describes the association line of the instance level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["The simple line without any diamond-shaped mark on its both ends",
                    "The line with a white diamond-shaped mark on one of its ends.",
                    "The line with a black diamond-shaped mark on one of its ends.",
                    "The line with a white triangle-shaped mark on one of its ends."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 68
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the aggregation of the class diagram of the class-based modeling of requirements modeling? …. (   )",
            choice: ["It is a “part-part” relationship.",
                    "It is a link between two classes.",
                    "It shows a class consisting of multiple classes.",
                    "It is more specific than association."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 69
        let qb = OldQuestion
        {
            question: "Which one of the following describes the aggregation line of the instance level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["The simple line without any diamond-shaped mark on its both ends",
                    "The line with a white diamond-shaped mark on one of its ends.",
                    "The line with a black diamond-shaped mark on one of its ends.",
                    "The line with a white triangle-shaped mark on one of its ends."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 70
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the composition of the class diagram of the class-based modeling of requirements modeling? …. (   )",
            choice: ["It shows strong dependency between instances of container class and instances of contained class(es).",
                    "If the container is destroyed, every contained instance is destroyed.",
                    "More specific than aggregation.",
                    "A contained class is a specialized form of its container class."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 71
        let qb = OldQuestion
        {
            question: "Which one of the following describes the composition line of the instance level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["The simple line without any diamond-shaped mark on its both ends",
                    "The line with a white diamond-shaped mark on one of its ends.",
                    "The line with a black diamond-shaped mark on one of its ends.",
                    "The line with a white triangle-shaped mark on one of its ends."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 72
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the class level relationships of a class-based modeling of requirements modeling? ……. (   ,   )",
            choice: ["Globalization",
                    "Generalization",
                    "Realization",
                    "Localization"],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 73
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the generalization inheritance of the class level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["It is a “has-a” relationship.",
                    "It is an “is-a” relationship.",
                    "A subclass is a specialized form of its superclass.",
                    "The superclass is the generalization of its subclasses."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 74
        let qb = OldQuestion
        {
            question: "Which one of the following describes the inheritance line of the class level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["The simple line without any diamond-shaped mark on its both ends",
                    "The line with a white diamond-shaped mark on one of its ends.",
                    "The line with a black diamond-shaped mark on one of its ends.",
                    "The line with a white triangle-shaped mark on one of its ends."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 75
        let qb = OldQuestion
        {
            question: "Which one of the following does not describe the realization of the class level relationships of a class-based modeling of requirements modeling? ……. (   )",
            choice: ["It is an implementation relationship.",
                    "All the behaviors are usually defined in detail in a supplier element.",
                    "It connects a client element with a supplier element",
                    "A client model element realizes (or implements, executes) the behavior of the supplier model elements specified."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 76
        let qb = OldQuestion
        {
            question: "Which one of the following is wrong in the requirement validation? ……. (   )",
            choice: ["Requirements must be consistent with system objectives.",
                    "Each requirement must be consistent with each other.",
                    "Not all requirements have to be completely specified in reality.",
                    "Missing requirements will cause problems in later phases."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 77
        let qb = OldQuestion
        {
            question: "Which one of the following is wrong in the requirement validation? ……. (   )",
            choice: ["You cannot remove all the ambiguities in requirements.",
                    "All requirements must be achievable technically.",
                    "You have to ask “Is the use of models adequate?”.",
                    "You have to ask “Are all requirements verifiable?”."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 78
        let qb = OldQuestion
        {
            question: "Which one of the following is the wrong description about the domain analysis? ……. (   )",
            choice: ["It is the activity to analyze the domain of the system being built.",
                    "It is the activity to collect requirements required by domain.",
                    "Users often do not recognize or state domain requirements.",
                    "For software development, domain analysis is optional but it is good if you perform domain analysis."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 79
        let qb = OldQuestion
        {
            question: "Which one of the following is the wrong description about the requirements management? ……. (   )",
            choice: ["Requirements must be complete and consistent.",
                    "New requirements may be introduced during the software processes.",
                    "Traceability – Relationships and/or links between requirements must be kept and maintained.",
                    "Requirements from different stakeholders are always the same."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 80
        let qb = OldQuestion
        {
            question: "Which one of the following is not the reason why requirements can be changed in the requirements management? ……. (   )",
            choice: ["Business environment changes",
                    "Daily weather changes",
                    "Technological changes",
                    "Customer requirements changes"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 81
        let qb = OldQuestion
        {
            question: "Which one of the following is not the description of requirements specification with natural language? ……. (   )",
            choice: ["It is an easy and natural way of describing requirements.",
                    "It can be unclear.",
                    "It is the best way to describe requirements.",
                    "The different levels of abstraction can be mixed."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 82
        let qb = OldQuestion
        {
            question: "Which ones of the following are the correct descriptions about system design? ……. (   ,   )",
            choice: ["It is the process of defining the architecture, components, modules, interfaces, and data for a system to satisfy system requirements.",
                    "It is involved with software, hardware, other interacting systems, and people",
                    "It is the process of satisfying system requirements.",
                    "It is involved with stakeholders, developers, and other relevant users."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 83
        let qb = OldQuestion
        {
            question: "Which one of the following is the incorrect description about software design? ……. (   )",
            choice: ["It is the process of implementing software solutions to one or more problems.",
                    "It is focused on HOW to implement the requirements into software in terms of business needs, technical considerations and the software quality.",
                    "It is involved with stakeholders, developers, and other relevant users.",
                    "The interaction mechanism with other systems is provided by software, but the mechanism complies with the system design."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 84
        let qb = OldQuestion
        {
            question: "Which one of the following is not the reason why we should design software before actual implementation of the software? ……. (   )",
            choice: ["In order to confirm that all requirements are being satisfied.",
                    "Because design enables tasks to be prioritized systematically.",
                    "Without the design, requirements change in later phases will cause difficulties and problems to comply with.",
                    "Design is a work as a clue for maintenance activities."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 85
        let qb = OldQuestion
        {
            question: "Which one of the following is not the reason why we should design software before actual implementation of the software? ……. (   )",
            choice: ["Design can model the software to be built.",
                    "Design models can be assessed for quality and improved before code is generated and tested.",
                    "Without a design, software quality would be harder to be assured.",
                    "Design always guarantees that software would be implemented and delivered on time."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 86
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct according to quality guidelines of design concept? ……. (   )",
            choice: ["A design must present the philosophy of software.",
                    "A design must be modular.",
                    "Software must be decomposed into smaller units",
                    "Smaller units are easier to manipulate."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 87
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct according to quality guidelines of design concept? ……. (   )",
            choice: ["Data, architecture, interfaces, and components must be designed using appropriate representation tools.",
                    "A design must maintain the complexity of components, connections and interfaces with external elements as simple as possible.",
                    "Complicated designs are always bad.",
                    "A design must be described using a notation so that its meaning is clearly and unambiguously understood."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 88
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct according to design principles? ……. (   )",
            choice: ["The design process should not suffer from “tunnel vision.”",
                    "Alternative approaches must not be considered and compared.",
                    "The design should be traceable to the analysis model.",
                    "The design provides clues about how requirements are satisfied."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 89
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect according to design principles? ……. (   )",
            choice: ["The design must not reinvent the wheel.",
                    "Use of design patterns - as an alternative to reinvention.",
                    "If you use design pattern, you don’t have to design the software ",
                    "The design should “minimize the intellectual distance” between the software and the problem."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 90
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect according to design principles? ……. (   )",
            choice: ["The structure of the software design should mimic the structure of the problem domain.",
                    "The design should exhibit uniformity and integration.",
                    "Standards of style and format should be defined for a design team for uniformity.",
                    "The design should be structured not to accommodate change."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 91
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect according to design principles? ……. (   )",
            choice: ["The design should be structured to degrade gently.",
                    "Software must be robust unless unusual circumstances are encountered.",
                    "Design is not coding, coding is not design.",
                    "The level of abstraction of the design model is higher than source code."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 92
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect according to design principles? ……. (   )",
            choice: ["The design should be assessed for quality as it is being created, not after requirement engineering.",
                    "Apply design concepts and design measures.",
                    "The design should be reviewed to minimize conceptual (semantic) errors.",
                    "Major conceptual elements of the design (omissions, ambiguity, inconsistency) must be checked."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 93
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about the abstraction levels of design? ……. (   ,   )",
            choice: ["The highest level of abstraction: most detailed.",
                    "The highest level of abstraction: most general.",
                    "The lowest level of abstraction: most detailed.",
                    "The lowest level of abstraction: most general."],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 94
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about modularity of design concept? ……. (   )",
            choice: ["Overall software is decomposed or partitioned into components until further decomposition is meaningless.",
                    "It is extremely hard, even impossible, to develop large-scale software without modularization.",
                    "Modularization makes the software manageable.",
                    "Modularization makes the software complicated."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 95
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about modularity of design concept? ……. (   )",
            choice: ["Modularization makes the software understandable.",
                    "Modularization causes cost reduction.",
                    "Module integration cost is imposed.",
                    "Modularization makes the software easy to use."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 96
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the design pattern? ……. (   )",
            choice: ["It is a solution to a recurring problem in a context.",
                    "It is the best way to represent knowledge and experience of experts.",
                    "It is not reusable.",
                    "It is the essence of design solutions with implementation methods."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 97
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the information hiding in the design concept? ……. (   )",
            choice: ["Information hiding means to hide the implementation details.",
                    "Information hiding lets you make use of controlled interfaces.",
                    "Use of global data is recommended.",
                    "It leads to better module independences."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 98
        let qb = OldQuestion
        {
            question: "Which one of the following describes the refactoring in the design concept? ……. (   )",
            choice: ["It is the process of enhancing the internal structure of a module, subsystems, and overall structure leaving external behavior as is.",
                    "It is the process of enhancing the external structure of a module, subsystems, and overall structure leaving internal behavior as is.",
                    "It is the process of enhancing the internal structure of a module, subsystems, and overall structure improving external behavior.",
                    "It is the process of enhancing the external structure of a module, subsystems, and overall structure improving internal behavior."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 99
        let qb = OldQuestion
        {
            question: "Which one of the following is not what to check to remove or improve in the refactoring in the design concept? ……. (   )",
            choice: ["Redundant elements",
                    "Inefficiency",
                    "Inappropriate data structure and algorithm",
                    "Too useful methods"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 100
        let qb = OldQuestion
        {
            question: "Which ones of the following describe the level of inter- and intra-module relationships? ……. (   ,   )",
            choice: ["Cohesion: an indication of the degree of unification among elements in a module",
                    "Coupling: an indication of the degree of the dependence between two modules",
                    "Coupling: an indication of the degree of unification among elements in a module",
                    "Cohesion: an indication of the degree of the dependence between two modules"],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 101
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about cohesion in the design concept? ……. (   )",
            choice: ["If a module carries out exactly one function, it is acceptable.",
                    "If the output of one function immediately becomes the input to the next function, it is unacceptable.",
                    "If two functions share the same data and the order of their executions must be kept, i.e. one after another, it is acceptable.",
                    "If two modules share the same data and the order of their executions does not matter, it is acceptable."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 102
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about cohesion in the design concept? ……. (   )",
            choice: ["If no data is shared by two modules and the order of their executions must be kept, it is sometimes acceptable but normally unacceptable.",
                    "If multiple activities are traditionally carried out at the same time without meaningful relationships, it is unacceptable.",
                    "If no data is shared by two modules and the order of their executions is not important, it is acceptable.",
                    "If several related but mutually exclusive tasks are carried out in a module, it is unacceptable."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 103
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about cohesion in the design concept? ……. (   )",
            choice: ["If more than one cohesion is involved in a module, you have selected the best one as the module’s level of cohesion.",
                    "Acceptable cohesions involved would not be problems.",
                    "The worst cohesion will cause problems.",
                    "If you find any unacceptable cohesion in software design, you must alter it so that it becomes an acceptable cohesion."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 104
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about coupling in the design concept? ……. (   )",
            choice: ["The coupling which occurs when the communication between two modules are accomplished only by parameters and with single field parameters only is acceptable.",
                    "The coupling that occurs when the communication between two modules are accomplished by parameters and one or more of them are composite data, such as arrays or records is unacceptable.",
                    "The coupling that occurs when one module controls the internal logic of the other, especially with flags, is unacceptable.",
                    "The coupling that occurs when two modules refer to the common area or global variables is unacceptable."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 105
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about coupling in the design concept? ……. (   )",
            choice: ["If a callee does not need knowledge of its caller’s definition and/or declaration of data structure being passed, it is acceptable coupling.",
                    "If a callee must have the knowledge of its caller’s definition and/or declaration of data structure being passed, it is unacceptable coupling.",
                    "If either a caller or a callee may pass flags to control the internal flow of execution of the other, it is unacceptable coupling.",
                    "If a module which receives a flag cannot execute instructions according to its own flow of control independently because it cannot decide which path it must go through, it is unacceptable coupling."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 106
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about coupling in the design concept? ……. (   )",
            choice: ["When a flag is upward and includes the information of the state of the system, error conditions, or other reporting information like End-of-File, End-of-Line, it is an unacceptable coupling.",
                    "If the values of global variables are referred to by more than one module, it is unacceptable coupling.",
                    "If the values of global variables are referred to by more than one module, it causes problems when errors are involved with the global variables.",
                    "If the values of global variables are referred to by more than one module, it takes time and effort to clarify which module assigns a wrong value to the global variable when the behavior of software goes wrong."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 107
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about coupling in the design concept? ……. (   )",
            choice: ["If a flag is downward, it is always considered as an acceptable coupling.",
                    "In coupling, global variables make it very complicated to trace references to them.",
                    "If you use global variables, it makes debugging activities very complicated.",
                    "When software is modified, all modules involved with the global variables must be checked carefully and completely."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 108
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about coupling in the design concept? ……. (   )",
            choice: ["The coupling which occurs when a module refers to the inside of another module is unacceptable.",
                    "If there is a goto statement to the statement inside another module, it is not acceptable.",
                    "If a module changes parts of code inside of another module, it is unacceptable.",
                    "Most modern high level languages allow a module to change parts of code inside of another module or make it easy to directly refer to the inside of another module so that it is easy to violate the scope of reference."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 109
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design concept? ……. (   )",
            choice: ["Compatibility: the software is able to operate with other products that are designed for interoperability with another product.",
                    "Extensibility: new capabilities can be added to the software with major changes to the underlying architecture.",
                    "Fault-tolerance: the software is resistant to and able to recover from component failure.",
                    "Maintainability: a measure of how easily bug fixes or functional modifications can be accomplished."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 110
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design concept? ……. (   )",
            choice: ["High maintainability can be the product of modularity and extensibility.",
                    "Modularity: the resulting software comprises well defined, independent components and it leads to better maintainability.",
                    "Modularity: the components could be then implemented and tested in isolation before being integrated to form a desired software system.",
                    "Modularity prevents division of work in a software development project."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 111
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design concept? ……. (   )",
            choice: ["Reliability: the software is able to perform a required function under stated conditions for a specified period of time.",
                    "Reusability: the software is able to add further features and modification with slight or no modification.",
                    "Robustness: the software is able to operate under stress or tolerate unpredictable or invalid input.",
                    "Robustness: the software can be designed with a resilience to big memory conditions."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 112
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design concept? ……. (   )",
            choice: ["Security: the software is able to withstand hostile acts and influences.",
                    "Usability: the software user interface must be usable for its target user/audience.",
                    "Usability: default values for the parameters should not be chosen because they are not a good choice for the majority of the users.",
                    "Performance: the software performs its tasks within a user-acceptable time."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 113
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design concept? ……. (   )",
            choice: ["Robustness: the software can work even under powered-cut situations.",
                    "Performance: the software does not consume too much memory.",
                    "Portability: the usability of the same software in different environments.",
                    "Scalability: the software adapts well to increasing data or number of users."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 114
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to structured design? . (   ,   )",
            choice: ["Object-oriented design",
                    "Layered design",
                    "Top-down design",
                    "Bottom-up design"],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 115
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design methodology? ……. (   )",
            choice: ["Structured Design: place logically similar elements together into physically different places.",
                    "Top-down design: the breaking down of a system to gain insight into its compositional sub-systems.",
                    "Bottom-up design: group elements that can be categorized as a family in terms of their functionalities.",
                    "Both top-down and bottom-up designs are accomplished in accordance with the level of abstraction."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 116
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect in the design methodology? ……. (   )",
            choice: ["Top-down design: decompose a problem into smaller problems until further decomposition is meaningless.",
                    "Bottom-up design: grouping of systems to construct more complex systems.",
                    "Object-Oriented Design: the process of planning a system of interacting objects.",
                    "Objects are types while classes are instances of the objects."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 117
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about information hiding in the design methodology? ……. (   )",
            choice: ["If information hiding is implemented well, users of a software component (such as a class) need to know only the essential details of how to initialize and access the component.",
                    "If information hiding is implemented well, users of a software component (such as a class) need to know the details of the implementation.",
                    "If information hiding is implemented well, it should hide all the details of an object that do not contribute to its essential characteristics.",
                    "If information hiding is implemented well, the structure of an object is hidden, as well as the implementation of its methods."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 118
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about information hiding in the design methodology? ……. (   )",
            choice: ["The terms “information hiding” and “encapsulation” are usually interchangeable.",
                    "If information hiding is implemented well, communications between objects are carried out by the “message passing.”",
                    "If information hiding is implemented well, the module works as a white box.",
                    "If information hiding is implemented well, the message passing is implemented as a function call."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 119
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about encapsulation in the design methodology? ……. (   )",
            choice: ["If encapsulation is implemented well, the implementation detail is visible to the other objects.",
                    "If encapsulation is implemented well, attributes and methods are put together in an object.",
                    "If encapsulation is implemented well, methods are the only way to access the internal data (attributes) of an object.",
                    "If encapsulation is implemented well, the internal state of an object is “encapsulated.”"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 120
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about encapsulation in the design methodology? ……. (   )",
            choice: ["If encapsulation is implemented well, the representation of the state of an object is visible to outside of the object.",
                    "If encapsulation is implemented well, message passing is the only way for methods of the receiving object to be executed.",
                    "If encapsulation is implemented well, attributes and methods are encapsulated in a class.",
                    "If encapsulation is implemented well, polymorphism enables different implementation details encapsulated in a concept."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 121
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about encapsulation in the design methodology? ……. (   )",
            choice: ["Encapsulation is implemented by inheritance.",
                    "All the attributes declared as public will be safely encapsulated.",
                    "The access to attributes of an object is encapsulated so that the access to the attributes is possible only by the message-passing.",
                    "All the attributes declared as protected will be safely encapsulated to its subclasses."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 122
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about inheritance in the design methodology? ……. (   )",
            choice: ["A subclass inherits the attributes and methods of its super-class.",
                    "A subclass may add new attributes or methods of its own.",
                    "It is intended to reuse methods defined in the super-classes without or with little modification",
                    "A subclass can refine or redefine only methods of its superclasses (ancestors)."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 123
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about inheritance in the design methodology? ……. (   )",
            choice: ["A superclass is a generalization of its sub-classes.",
                    "A subclass is a specialization of its superclass.",
                    "Implementation reuse means reuse of the code defined by the subclasses.",
                    "Interface reuse means polymorphism."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 124
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about inheritance in the design methodology? ……. (   )",
            choice: ["Super-class inherits from its subclass.",
                    "Reuse of the interface defined by abstract classes means delegating concrete operations to subclasses",
                    "Superclass is a synonym of child class.",
                    "Subclass is a synonym of parent class."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 125
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about polymorphism in the design methodology? ……. (   )",
            choice: ["Subclass determines interface while superclass determines implementation.",
                    "One name, same form",
                    "It creates one function with the same name and same argument list.",
                    "The function to be invoked will be determined according to the created instances of subclasses."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 126
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about overloading in the design methodology? ……. (   )",
            choice: ["One name, multiple forms",
                    "Create more than one function with the same name but different argument list.",
                    "Overloaded functions can be distinguished by their return type.",
                    "The function to be invoked will be determined according to the type and the number of arguments in the function."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 127
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about overloading in the design methodology? ……. (   )",
            choice: ["It is a type of polymorphism",
                    "It is the use of same thing for different purposes",
                    "The compiler checks the type and the number of parameters to distinguish among multiple functions with the same name.",
                    "If the functions which have the same name, the same number of parameters, the same types of parameters and the different return types are all overloaded functions that are distinguished from each other."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 128
        let qb = OldQuestion
        {
            question: "Which one of the following is the incorrect reason why we have to design architecture in the architecture design? ……. (   )",
            choice: ["To analyze whether the design effectively comply with requirements",
                    "To maximize the price of the software products",
                    "To consider architectural alternatives",
                    "To reduce the risk of software development failure"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 129
        let qb = OldQuestion
        {
            question: "Which one of the following is the incorrect reason why we have to design architecture in the architecture design? ……. (   )",
            choice: ["To communicate with stakeholders",
                    "To ensure whether non-functional requirements are being realized",
                    "To evaluate user requirements",
                    "To enhance the reusability, especially for the large scale reusability"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 130
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the list of what we have to consider when we design architecture? ……. (   )",
            choice: ["Beauty",
                    "Performance",
                    "Security",
                    "Safety"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 131
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the list of what we have to consider when we design architecture? ……. (   ,   )",
            choice: ["Confidentiality",
                    "Availability",
                    "Affordability",
                    "Maintainability"],
            group: num,
            answer: 0x42,
        };
        questions.push(qb);

        num += 1;  // num = 132
        let qb = OldQuestion
        {
            question: "Which one of the following is not the activity to enhance performance when we design architecture? ……. (   )",
            choice: ["Use global variables as much as possible.",
                    "Localize critical operations.",
                    "Minimize communications.",
                    "Use large rather than fine-grain components."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 133
        let qb = OldQuestion
        {
            question: "Which one of the following is the activity to enhance security when we design architecture? ……. (   )",
            choice: ["Use a layered architecture with critical assets in the outer layers to protect them.",
                    "Use encapsulation to place critical assets in public scope.",
                    "Use a layered architecture with critical assets in the inner layers to protect them.",
                    "Use information hiding to expose critical assets to the outside."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 134
        let qb = OldQuestion
        {
            question: "Which one of the following is the activity to enhance safety when we design architecture? ……. (   )",
            choice: ["Spread safety-uncritical features over a large number of subsystems.",
                    "Spread safety-critical features over a large number of subsystems.",
                    "Localize safety-uncritical features in a small number of subsystems.",
                    "Localize safety-critical features in a small number of subsystems."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 135
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the activities to enhance availability when we design architecture? ……. (   ,   )",
            choice: ["We may include redundant components to enhance availability.",
                    "We should get rid of all redundant components to enhance availability.",
                    "We should make software to be cheap and affordable to enhance availability.",
                    "Mechanisms for fault tolerance are included for enhanced availability."],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);

        num += 1;  // num = 136
        let qb = OldQuestion
        {
            question: "Which one of the following is the activity to enhance maintainability when we design architecture? ……. (   )",
            choice: ["Use redundant components to enhance maintainability.",
                    "Use fine-grain, replaceable components to enhance maintainability.",
                    "Spread safety-critical features over a large number of subsystems.",
                    "Use old technology to enhance maintainability.  "],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 137
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about data-centered architecture in the architectural design? ……. (   )",
            choice: ["Data repository is the center of the architecture.",
                    "Shared data stored in the repository are frequently updated, inserted, deleted by other components.",
                    "Efficient distribution is easy.",
                    "It is also known as the Repository Model."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 138
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about data-centered architecture in the architectural design? ……. (   )",
            choice: ["Efficient to share data, especially when the amount of data is large",
                    "Subsystems need knowledge about the management of the system.",
                    "Subsystems must follow the repository data model.",
                    "Data evolution is difficult and requires high costs."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 139
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about data flow architecture in the architectural design? ……. (   )",
            choice: ["It is also known as Function-Oriented Pipelining",
                    "Input data is transformed to output data through a series of computation and manipulation.",
                    "Communication with stakeholders is pretty hard.",
                    "The output from a component becomes the input to the immediately following components. - Pipelining"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 140
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about data flow architecture in the architectural design? ……. (   )",
            choice: ["It enhances the reusability.",
                    "It can be simply applied as either a concurrent or a sequential system.",
                    "Data transfer format must be standardized.",
                    "It is easy to support event-based (event-driven) interaction"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 141
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Call and return architecture in the architectural design? ……. (   )",
            choice: ["A system is decomposed into subsystems.",
                    "Each subsystem is decomposed into modules until further decomposition is meaningless.",
                    "Each subsystem and module must be as independent as possible.",
                    "Outer nodes (modules) usually manage workflows."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 142
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Call and return architecture in the architectural design? ……. (   )",
            choice: ["Leaves do the actual tasks.",
                    "Fan-out degree is limited to 7±2.",
                    "Decomposition makes the fan-out degree higher.",
                    "High fan-in degree is desirable."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 143
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Object-oriented architecture in the architectural design? ……. (   )",
            choice: ["It still needs well-defined global variables.",
                    "It needs to identify Classes and their attributes and methods.",
                    "It needs to structure the loosely coupled classes with well-defined interfaces.",
                    "Objects are created from the classes for the implementation."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 144
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Client-server architecture in the architectural design? ……. (   )",
            choice: ["It belongs to data-centered system model",
                    "Processing is distributed across a range of components.",
                    "Stand-alone servers provide services upon requests.",
                    "Client systems request services to servers."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 145
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Client-server architecture in the architectural design? ……. (   )",
            choice: ["Data distribution is easy.",
                    "Standalone systems are effectively used.",
                    "It may not require high performance hardware.",
                    "It is easy to add new servers"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 146
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Client-server architecture in the architectural design? ……. (   )",
            choice: ["It is easy to upgrade existing servers",
                    "Subsystems may use different data organizations.",
                    "Additional management is required in each client.",
                    "The types and kinds of services and corresponding service providers (i.e. servers) are difficult to identify."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 147
        let qb = OldQuestion
        {
            question: "Which ones of the following are the models of Centralized architecture in the architectural design? ……. (   ,   )",
            choice: ["Waterfall model",
                    "Evolutionary model",
                    "Call and Return model",
                    "Manager Model"],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 148
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about the Call and Return model of Centralized architecture in the architectural design? … (   ,   )",
            choice: ["Root module initiates execution and controls move downwards.",
                    "One system controls the processes of other systems.",
                    "It is applicable to sequential systems.",
                    "It is applicable to concurrent systems."],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 149
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about the Manager model of Centralized architecture in the architectural design? ……. (   ,   )",
            choice: ["Root module initiates execution and controls move downwards.",
                    "One system controls the processes of other systems.",
                    "It is applicable to sequential systems.",
                    "It is applicable to concurrent systems."],
            group: num,
            answer: 0x42,
        };
        questions.push(qb);

        num += 1;  // num = 150
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about Event-driven architecture in the architectural design? ……. (   )",
            choice: ["It is initiated by external events.",
                    "It is initiated by internal events.",
                    "The occurrence of an event initiates the process of subsystems that processes the event.",
                    "The occurrence of an event results from the process of subsystems that processes the event."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 151
        let qb = OldQuestion
        {
            question: "Which ones of the following are two types of Event-driven architecture in the architectural design? ……. (   ,   )",
            choice: ["Broadcasting architecture",
                    "Object-oriented architecture",
                    "Centralized architecture",
                    "Interrupt-driven architecture"],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);

        num += 1;  // num = 152
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Broadcasting architecture in the architectural design? ……. (   )",
            choice: ["An event is broadcast to all subsystems by the event-handler.",
                    "A subsystem that can issue the event takes the event to process.",
                    "It is effective for systems integrated with subsystems or modules through a network",
                    "Subsystems register specific events that can be handled by themselves."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 153
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Interrupt-driven architecture in the architectural design? ……. (   )",
            choice: ["It is applicable to batch processing systems for fast responses",
                    "Interrupts are detected by the interrupt handler.",
                    "Then the interrupt is passed to a subsystem (or a module) for processing.",
                    "Interrupt types are known to the interrupt handler."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 154
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Interrupt-driven architecture in the architectural design? ……. (   )",
            choice: ["Each interrupt type is assigned to a memory location.",
                    "A hardware switch transfers an interrupt to the corresponding memory location.",
                    "It enables batch processings.",
                    "The implementation is complicated and hard to test."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 155
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Algorithm design in Structured Component Design? ……. (   )",
            choice: ["Stepwise refinement should be done from the highest level of abstraction to the lowest, up to the level of code.",
                    "Quality must be considered.",
                    "The possibility of reuse must be addressed.",
                    "Domain engineering activities must be accompanied."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 156
        let qb = OldQuestion
        {
            question: "Which one of the following is correct about Data structure design in Structured Component Design? ……. (   )",
            choice: ["It should be independent from the algorithm.",
                    "It has nothing to do with the algorithm design.",
                    "It is also known as data encapsulation.",
                    "It must be appropriate for the algorithms."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 157
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about Object-Oriented Component Design? ……. (   )",
            choice: ["Cohesion and Coupling are good means to measure the quality of modularization.",
                    "Cohesion and Coupling are good means to measure the quality of integration.",
                    "Some levels of both Cohesion and Coupling are considered to be better than the others.",
                    "All the levels of both Cohesion and Coupling are considered to be the same."],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 158
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about Object-Oriented Component Design? ……. (   )",
            choice: ["Usually, a level-up of both cohesion and coupling to the highest level is recommended.",
                    "Both cohesion and coupling should be leveled up to the highest level until further level-up is impossible.",
                    "Sometimes, the level-up of both cohesion and coupling is not always possible.",
                    "A meaningless level-up, i.e. especially when the level-up is the only purpose without any benefit, is not recommended."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 159
        let qb = OldQuestion
        {
            question: "Which one of the following is the best cohesion in  Object-Oriented Component Design? ……. (   )",
            choice: ["A class consists of methods that do not have any reason to be included in the class together other than convenience.",
                    "A series of methods are kept in the same class where the output from a module becomes the input to the immediately next module.",
                    "All codes for the system start up, shut down, initialization, or clean up etc. are put together in a method.",
                    "Data structures and the manipulation of them are completely hidden from the user."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 160
        let qb = OldQuestion
        {
            question: "Which one of the following is the best coupling in  Object-Oriented Component Design? ……. (   )",
            choice: ["A component modifies or uses the “internal” data or code of another component.",
                    "Component communication is accomplished only via parameters or message passing.",
                    "One component affects the sequence of execution in another.",
                    "Components share data using a global variable and thus become dependent on each other."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 161
        let qb = OldQuestion
        {
            question: "Which one of the following is difficult to consider as a design error of interface design? ……. (   )",
            choice: ["Lack of consistency",
                    "Too little memorization",
                    "No guidance or help",
                    "No context sensitivity"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 162
        let qb = OldQuestion
        {
            question: "Which ones of the following are considered as a design error of interface design? ……. (   ,   )",
            choice: ["Poor response",
                    "Consistency",
                    "Context sensitivity",
                    "Arcane or unfriendly"],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);

        num += 1;  // num = 163
        let qb = OldQuestion
        {
            question: "Which one of the following does not fit the golden rule of interface design? ……. (   )",
            choice: ["Provide what the user recognizes naturally.",
                    "Maximize the user’s memory load by avoiding establishing meaningful defaults and shortcuts.",
                    "Make the interface consistent.",
                    "Do not force a user into unnecessary or undesired actions."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 164
        let qb = OldQuestion
        {
            question: "Which one of the following does not fit the golden rule of interface design? ……. (   )",
            choice: ["Provide flexible interactions.",
                    "User interactions should be interruptible and undoable.",
                    "Streamline interaction as skill levels advance.",
                    "The interaction cannot be customized."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 165
        let qb = OldQuestion
        {
            question: "Which one of the following does not fit the golden rule of interface design? ……. (   )",
            choice: ["Hide technical details from the casual user.",
                    "Objects on the screen must be directly used.",
                    "Reduce demands that must be memorized.",
                    "Don’t use meaningful default values."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 166
        let qb = OldQuestion
        {
            question: "Which one of the following does not fit the golden rule of interface design? ……. (   )",
            choice: ["Shortcuts must be intuitively recognized.",
                    "The visual layout must be based on a real world feature.",
                    "Interface layout should be the same across the various cultures.",
                    "Make the interface consistent."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 167
        let qb = OldQuestion
        {
            question: "Which ones of the following are considered to fit the golden rule of interface design? ……. (   ,   )",
            choice: ["Maintain consistency across a family of applications.",
                    "Shortcuts should be avoided because it increases the users’ memory load.",
                    "If users feel comfortable with and accustomed to the past user interface, do not recreate unless there is a serious reason to do so.",
                    "Don’t use meaningful default values."],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 168
        let qb = OldQuestion
        {
            question: "Which one of the following is not really needed to consider when modeling the interface design? ……. (   )",
            choice: ["A characteristic of all end users of the system",
                    "A characteristic of all the stakeholders",
                    "A design realization of the user model",
                    "System perception"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 169
        let qb = OldQuestion
        {
            question: "Which ones of the following are needed to consider when modeling the interface design? ……. (   ,   )",
            choice: ["The benefits of all the potential stakeholders",
                    "The stakeholder’s typical understanding of the interface",
                    "The user’s mental image of what the interface is",
                    "“Look and feel” coupled with information about the interface syntax and semantics"],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 170
        let qb = OldQuestion
        {
            question: "Which one of the following is not really needed to understand when processing the interface design? ……. (   )",
            choice: ["The end-users who will interact with the system through the interface",
                    "The tasks that end-users must do",
                    "The content that is implicitly presented in the manual",
                    "The environment in which the tasks are conducted"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 171
        let qb = OldQuestion
        {
            question: "Which one of the following is not really needed to analyze when analyzing the user in order to perform interface design? ……. (   )",
            choice: ["User’s level of English proficiency",
                    "User’s level of computer skill",
                    "User’s level of education",
                    "User’s level of expertise about the task"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 172
        let qb = OldQuestion
        {
            question: "Which one of the following is not really needed to analyze when analyzing the user in order to perform interface design? ……. (   )",
            choice: ["User’s language",
                    "The highest computer performance of users",
                    "Desired training methods for users",
                    "User’s input preference"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 173
        let qb = OldQuestion
        {
            question: "Which one of the following is not really needed to analyze when analyzing the user in order to perform interface design? ……. (   )",
            choice: ["Desired average learning curve of users",
                    "Whether the software is a main tool or supplementary",
                    "The age range of the user",
                    "Expected consequences of the user’s mistake"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 174
        let qb = OldQuestion
        {
            question: "Which one of the following is the incorrect description of the task analysis in order to perform interface design? ……. (   )",
            choice: ["Use-cases define basic interaction.",
                    "Workflow analysis defines how a work process is completed when a single person (and role) is involved.",
                    "Task elaboration refines interactive tasks.",
                    "Object elaboration identifies interface objects (classes)."],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 175
        let qb = OldQuestion
        {
            question: "Which one of the following questions is not really needed to ask when analyzing the task in order to perform interface design? ……. (   )",
            choice: ["What work will the user perform in specific circumstances?",
                    "What tasks and subtasks will be performed as the user does the work?",
                    "What specific problem domain objects will the user manipulate as work?",
                    "What is an unnecessary work task?"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 176
        let qb = OldQuestion
        {
            question: "Which ones of the following questions are needed to ask when analyzing the task in order to perform interface design? ……. (   ,   )",
            choice: ["What is the most favorite work task of the user?",
                    "What is the sequence of work tasks—the workflow?",
                    "What is the most demanding work task of the user?",
                    "What is the hierarchy of tasks?"],
            group: num,
            answer: 0x42,
        };
        questions.push(qb);

        num += 1;  // num = 177
        let qb = OldQuestion
        {
            question: "Which one of the following questions is not really needed to ask when analyzing the display contents in order to perform interface design? ……. (   )",
            choice: ["Are different types of data assigned to consistent geographic locations on the screen (e.g., photos always appear in the upper right hand corner)?",
                    "Can the user customize the screen location for content?",
                    "If a large report is to be presented, how should it be integrated for ease of understanding?",
                    "Is proper on-screen identification assigned to all content?"],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 178
        let qb = OldQuestion
        {
            question: "Which one of the following questions is not really needed to ask when analyzing the display contents in order to perform interface design? ……. (   )",
            choice: ["Will mechanisms be available for moving directly to detailed information for large collections of data?",
                    "Will graphical output be scaled to fit within the bounds of the display device that is used?",
                    "How will color be used to enhance understanding?",
                    "How will error messages and warnings be presented to the user?"],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 179
        let qb = OldQuestion
        {
            question: "Which one of the following is not the issue of user interface design? ……. (   )",
            choice: ["Response time",
                    "Help facilities",
                    "Error handling",
                    "User’s happiness"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 180
        let qb = OldQuestion
        {
            question: "Which one of the following is not the issue of user interface design? ……. (   )",
            choice: ["Menu and command labeling",
                    "Software performance",
                    "Application accessibility",
                    "Internationalization"],
            group: num,
            answer: 2,
        };
        questions.push(qb);
        Self { header, questions }
    }

    pub fn new_mis() -> Self
    {
        let header = Header::new(
            "Management Information Systems".to_string(),
            "Name".to_string(),
            "ID".to_string(),
            vec!["Type A".to_string(), "Type B".to_string()],
            "Notice:\n".to_string()
            + "* All the questions should be considered, understood and interpreted in the context of the Information Security course you learned. Otherwise, the questions may or may not make sense.\n"
            + "* Type A: Multiple Choice 1 – you have to choose one answer from the list.\n"
            + SAVE_PAPER_SPACE + "# If your answer is correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If your answer is incorrect, you will lose 1 point.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or more than one answer from the list, you will get 0 points.\n"
            + "* Type B: Multiple Choice 2 – you have to choose two answers from the list.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are correct, you will get 3 points.\n"
            + SAVE_PAPER_SPACE + "# If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.\n"
            + SAVE_PAPER_SPACE + "# If both answers that you chose are incorrect, you will lose 3 points.\n"
            + SAVE_PAPER_SPACE + "# If you choose nothing or one or more than two answers from the list, you will get 0 points.\n");
        
        let mut questions = Vec::<OldQuestion>::new();

        let mut num = 1u16;
        let mut qb = OldQuestion
        {
            question: "Which one of the following is correct about the definitions of information and data?  …… (   )",
            choice: ["Information can be defined as meaningfully interpreted data.",
                    "A piece of raw data such as a number 996-312-69-05-40 does make a lot of sense on its own.",
                    "Even if we say Tel: +996-312-69-05-40, it does not start making sense yet.",
                    "The terminologies ‘data’ and ‘information’ have the same meaning and they are synonyms to each other."],
            group: num,
            answer: 0x1,
        };
        questions.push(qb);

        num += 1;   // num = 2
        qb = OldQuestion
        {
            question: "Which ones of the following are correct about information?  …… (   ,   )",
            choice: ["From a system analyst's point of view, information is a sequence of symbols that can be construed to a useful message.",
                    "An Information System is a system that gathers data and disseminates information with the sole purpose of providing information to its users.",
                    "The main objective of an information system is to provide raw data to its users.",
                    "Information systems are all the same regardless of the type of users who use the system."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb); 

        num += 1;  // num = 5
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about data and information?  …… (   )",
            choice: ["Data can be described as well-processed facts and figures.",
                    "Plain collected data as raw facts cannot help in decision-making.",
                    "Data is the raw material that is organized, structured, and interpreted to create useful information systems.",
                    "Data is defined as 'groups of random symbols in the form of text, images, voice representing quantities, action and objects'."],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 6
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct about data and information?  …… (   )",
            choice: ["Information is uninterpreted data.",
                    "Information is created from organized, structured, and processed data in a particular context.",
                    "Information is data that has been processed into a form that is meaningful to the recipient.",
                    "Information is of real or perceived value in the current or the prospective action or decision of the recipient."],
            group: num,
            answer: 0x1,
        };
        questions.push(qb);

        num += 1;  // num = 9
        let qb = OldQuestion
        {
            question: "Which one of the following is not correct for the most popular data collection techniques?  …… (   )",
            choice: ["Surveys − Questionnaires are prepared to collect the data from the field.",
                    "Secondary data sources or archival data: Data are collected through old records, magazines, company websites, etc.",
                    "Objective measures or tests − An experimental test is conducted on the subject and the data are collected.",
                    "Interviews − Data are collected by the system analyst by following a flexible procedure and collecting the answers to a set of random questions through personal interviews."],
            group: num,
            answer: 0x4,
        };
        questions.push(qb);

        num += 1;  // num = 10
        let qb = OldQuestion
        {
            question: "Based on Anthony's classification of Management, information used in business for decision-making is generally categorized into three types by characteristics. Which one of the following does not belong to these three types?  …… (   )",
            choice: ["Strategic Information",
                    "Planning Information",
                    "Tactical Information",
                    "Operational Information"],
            group: num,
            answer: 2,
        };
        questions.push(qb);

        num += 1;  // num = 14
        let qb = OldQuestion
        {
            question: "In terms of applications, information is generally categorized into six types by characteristics. Which ones of the following belong to these six types?  …… (   ,   )",
            choice: ["Strategic Information",
                    "Tactical Information",
                    "Planning Information",
                    "Control Information"],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 15
        let qb = OldQuestion
        {
            question: "In terms of applications, information is generally categorized into six types by characteristics. Which ones of the following belong to these six types?  …… (   ,   )",
            choice: ["Tactical Information",
                    "Knowledge Information",
                    "Organizational Information",
                    "Operational Information"],
            group: num,
            answer: 0x32,
        };
        questions.push(qb);

        num += 1;  // num = 16
        let qb = OldQuestion
        {
            question: "In terms of applications, information is generally categorized into six types by characteristics. Which ones of the following belong to these six types?  …… (   ,   )",
            choice: ["Functional/Operational Information",
                    "Strategic Information",
                    "Knowledge Information",
                    "Database Information"],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);

        num += 1;  // num = 23
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the quality of information?  …… (   )",
            choice: ["Information is a vital resource for the success of any organization.",
                    "Future of an organization lies in using and disseminating information wisely.",
                    "Good quality information placed in the right context at the right time tells us about opportunities and problems well in advance.",
                    "Good quality information − Quality is a value that would be constant regardless of the users and uses of the information."],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 24
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the most essential characteristic features for information quality?  …… (   )",
            choice: ["Deceptive − It should be full of errors and mistakes, ambiguous, and wrong.",
                    "Reliability − It should be verifiable and dependable.",
                    "Timely − It must be current and it must reach the users well in time, so that important decisions can be made in time.",
                    "Relevant − It should be current and valid information and it should reduce uncertainties."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 25
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the most essential characteristic features for information quality?  …… (   )",
            choice: ["Accurate − It should be free of errors and mistakes, true, and not deceptive.",
                    "Sufficient − It should be adequate in quantity, so that decisions can be made on its basis.",
                    "Lack − It does not have to meet all the needs in the current context.",
                    "Unambiguous − It should be expressed in clear terms, in other words, comprehensive."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 26
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the most essential characteristic features for information quality?  …… (   )",
            choice: ["Complete − It should meet all the needs in the current context.",
                    "Incomparable − It should be various in collection, analysis, content, and format.",
                    "Unbiased − It should be impartial, free from any bias. In other words, it should have integrity.",
                    "Explicit − It should not need any further explanation."],
            group: num,
            answer: 0x2,
        };
        questions.push(qb);

        num += 1;  // num = 27
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about the most essential characteristic features for information quality?  …… (   ,   )",
            choice: ["Comparable − It should be of uniform collection, analysis, content, and format.",
                    "Reproducible − It could be used by documented methods on the same data set to achieve a consistent result.",
                    "Incomparable − It should be various in collection, analysis, content, and format.",
                    "Lack − It does not have to meet all the needs in the current context."],
            group: num,
            answer: 0x21,
        };
        questions.push(qb);

        num += 1;  // num = 28
        let qb = OldQuestion
        {
            question: "Which ones of the following are correct about information?  …… (   ,   )",
            choice: ["Information processing has kept our society from being transformed.",
                    "Access to information and capability of information processing has helped in achieving perfection in accounting and other business processes.",
                    "Information is needed to survive in the modern competitive world.",
                    "Information is needed to create strong information systems and keep these systems up to date."],
            group: num,
            answer: 0x43,
        };
        questions.push(qb);

        num += 1;  // num = 29
        let qb = OldQuestion
        {
            question: "Which one of the following is incorrect about the Implications of Information in Business?  …… (   )",
            choice: ["Having information guarantees success in modern business.",
                    "Information processing has transformed our society in numerous ways.",
                    "From a business perspective, there has been a huge shift towards increasingly automated business processes and communication.",
                    "Access to information and capability of information processing has helped in achieving greater efficiency in accounting and other business processes."],
            group: num,
            answer: 1,
        };
        questions.push(qb);

        num += 1;  // num = 31
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the five main uses of information?  …… (   )",
            choice: ["Planning",
                    "Recording",
                    "Controlling",
                    "Overviewing"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 32
        let qb = OldQuestion
        {
            question: "Which ones of the following belong to the five main uses of information?  …… (   ,   )",
            choice: ["Measuring",
                    "Overviewing",
                    "Decision-making",
                    "Comparing"],
            group: num,
            answer: 0x31,
        };
        questions.push(qb);

        num += 1;  // num = 38
        let qb = OldQuestion
        {
            question: "Which one of the following factors arising as an outcome of information processing does not help speed up business events and achieve greater efficiency?  …… (   )",
            choice: ["Directly and immediate linkage to the system",
                    "Faster communication of an order",
                    "Electronic transfer of funds for faster payment",
                    "Electronically solicited quality (helps in determining the best quality)"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 39
        let qb = OldQuestion
        {
            question: "Which one of the following does not belong to the four-fold path that decision-making generally takes?  …… (   )",
            choice: ["Understanding the need for decision or the opportunity",
                    "Discarding alternative course of actions",
                    "Evaluating all alternative course of actions",
                    "Deciding the right path for implementation"],
            group: num,
            answer: 0x2,
        };
        questions.push(qb);

        num += 1;  // num = 40
        let qb = OldQuestion
        {
            question: "Type A]   Which one of the following is not why MIS Needs for Information Systems?  …… (   )",
            choice: ["Because MIS is an information system that provides information in the form of standardized reports and displays for the managers.",
                    "Because MIS is a broad class of information systems designed to provide information needed for effective decision making.",
                    "Because MIS is an absolute solution to provide information needed for best decision making in the form of beautifully decorated reports.",
                    "Because data and information created from an accounting information system and the reports generated thereon are used to provide accurate, timely and relevant information needed for effective decision making by managers."],
            group: num,
            answer: 3,
        };
        questions.push(qb);

        num += 1;  // num = 41
        let qb = OldQuestion
        {
            question: "Which one of the following is not the goal that management information systems provide information to support management decision making with?  …… (   )",
            choice: ["Pre-specified and preplanned reporting to managers",
                    "Interactive and ad-hoc support for decision making",
                    "Critical information for top management",
                    "Critical and interactive support for delivery on the ground"],
            group: num,
            answer: 4,
        };
        questions.push(qb);

        num += 1;  // num = 42
        let qb = OldQuestion
        {
            question: "Which ones of the following describe the vital importance of MIS is to any organization?  …… (   ,   )",
            choice: ["Because it emphasizes the management decision making, not only processing of data generated by business operations.",
                    "Because it does not emphasize the management decision making, but the processing of data generated by business operations.",
                    "Because it does not emphasize the systems framework, but organizing information systems applications.",
                    "Because it emphasizes the systems framework that should be used for organizing information systems applications."],
            group: num,
            answer: 0x41,
        };
        questions.push(qb);
        Self { header, questions }
    }
}

