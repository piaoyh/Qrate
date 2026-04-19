# qrate: A Rust Library for Examination Generation
`qrate` is a core engine designed to help educators effortlessly generate unique examination sets, making academic dishonesty significantly more difficult. Developed through a synergy of Human Intelligence (HI) and Artificial Intelligence (AI), every line of code in this crate reflects both AI-driven efficiency and rigorous human craftsmanship.

## Background
Teachers and lecturers often struggle with the integrity of exams. In environments where academic dishonesty is a concern, preventing students from copying from one another is a constant challenge.

This project started as a command-line tool I built to practice Rust and solve a real-world problem in my own teaching. To make this solution more accessible and professional, I have decoupled the core engine from the interface and published it as this crate.

## Purpose
One of the most effective ways to prevent cheating is to provide each student with a unique question set. However, manually creating dozens of different versions is an overwhelming task for any educator.

`qrate` automates this process. With this crate, you can develop a custom examination generator that creates hundreds of unique papers in seconds—all you have to do is hit print.

## How it Works
A generator built with `qrate` ensures that every student receives a distinct exam paper:

- Question Variety: A question in one student's set may not appear in another's.
- Randomized Order: Even if two students share the same question, it will appear at different positions in their respective exams.
- Shuffled Choices: For multiple-choice questions, the order of the answers is randomized for every paper.

__By introducing these layers of randomization, qrate makes it extremely difficult for students to share answers during an exam.__
