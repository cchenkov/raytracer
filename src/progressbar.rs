use std::io::{Write};

pub struct ProgressBar<'a, W: Write> {
    complete: usize,
    total: usize,
    length: usize,
    output: &'a mut W
}

impl<'a, W: Write> ProgressBar<'a, W> {
    pub fn new(total: usize, length: usize, output: &'a mut W) -> Self {
        ProgressBar {
            complete: 0,
            total,
            length,
            output
        }
    }

    pub fn increment(&mut self, step: usize) {
        self.complete = self.complete + step;

        let complete_length = (self.complete * self.length) / self.total;
        let incomplete_length = self.length - complete_length;
        let percentage = (self.complete * 100) / self.total;

        self.output.write_fmt(format_args!(
            "\r[{}{}] {}%", 
            "#".repeat(complete_length), 
            "-".repeat(incomplete_length), 
            percentage
        )).unwrap();

        self.output.flush().unwrap();
    }
}