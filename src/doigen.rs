use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
Gaurav Sablok,
gsablok@proton.me
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fasta {
    pub id: String,
    pub sequence: String,
}

pub fn generatetag(pathfile: &str) -> Result<String, Box<dyn Error>> {
    let fastaunpack: Vec<Fasta> = readfasta(pathfile).expect("file not present");
    for i in fastaunpack.iter() {
        let filename = format!("{}.{}", i.id, "id");
        let mut filename = File::create(filename).expect("file not present");
        writeln!(filename, "{}", i.sequence).expect("File not found");
    }

    let _ = Command::new("mkdir")
        .arg("tags")
        .output()
        .expect("commandfailed");

    let _ = Command::new("sh")
        .arg("./src/md5sum.sh")
        .output()
        .expect("folder not present");

    let fileopenadd = File::open("estimatetag.txt").expect("file not present");
    let fileopenreadadd = BufReader::new(fileopenadd);
    let mut sequenceid: Vec<String> = Vec::new();
    for i in fastaunpack.iter() {
        sequenceid.push(i.id.clone());
    }
    let mut doivec: Vec<String> = Vec::new();
    let mut doifinaltag: Vec<String> = Vec::new();
    for i in fileopenreadadd.lines() {
        let tagline = i.expect("line not found");
        let tagcollectstring = tagline
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .take(6)
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .concat()
            .to_string();

        let finaltagabout = tagline
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .skip(6)
            .take(6)
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .concat()
            .to_string();
        doivec.push(tagcollectstring);
        doifinaltag.push(finaltagabout);
    }

    let mut finaltag: Vec<_> = Vec::new();
    for i in 0..sequenceid.len() {
        let doitag = format!("{}/{}/{}", "doi.org", doivec[i], doifinaltag[i]);
        finaltag.push(doitag);
    }

    let mut finaltagwrite = File::create("doigenerated.txt").expect("file not present");
    for i in 0..finaltag.len() {
        writeln!(finaltagwrite, "{}\t{}", sequenceid[i], finaltag[i]).expect("File not found");
    }

    let _ = Command::new("rm")
        .arg("estimatetag.txt")
        .output()
        .expect("file not present");

    Ok("the doi for the sequences have been written".to_string())
}

pub fn readfasta(pathfile: &str) -> Result<Vec<Fasta>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut fastavec: Vec<Fasta> = Vec::new();
    let mut id: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            id.push(line.replace(">", ""));
        } else if !line.starts_with("#") {
            sequence.push(line);
        }
    }
    for i in 0..id.len() {
        fastavec.push(Fasta {
            id: id[i].clone().to_string(),
            sequence: sequence[i].clone().to_string(),
        })
    }

    Ok(fastavec)
}
