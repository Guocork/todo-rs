use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Record {
    pub id: i32,
    pub content: String,
}

pub struct Database {
    pub file: File,
}

impl Database {
    pub fn open(filename: &str) -> Database {
        let file = OpenOptions::new()
            .create(true) //这意味着如果文件不存在，则会创建该文件。
            .read(true) //这意味着文件将以读取模式打开。
            .write(true) //这意味着文件将以写入模式打开。
            .open(filename) //打开名为 filename 的文件。
            .unwrap();

        Database { file }
    }

    pub fn add_record(&mut self, record: &Record) {
        let line = format!("{},{}\n", record.id, record.content);
        writeln!(self.file, "{}", line).unwrap();
        println!("📝 Item added: {}", record.content);
    }

    // 解析记录行
    pub fn parse_record_line(line: &str) -> Record {
        let fields: Vec<&str> = line.split(',').collect();
        // 处理空行的情况
        if fields.len() == 1 {
            return Record {
                id: 0,
                content: "".to_string(),
            };
        }
        let content = fields[1..].join(",");
        Record {
            id: fields[0].parse::<i32>().unwrap(),
            content,
        }
    }
}
