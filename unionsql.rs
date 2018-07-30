use  std::io::{self,BufReader};
use  std::fs::read_dir;
use  std::path::Path;
use  std::fs::{File,OpenOptions};
use  std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() !=2 {
        writeln!(std::io::stderr(),"argument count is  wrong").unwrap();
        std::process::exit(1);
    }
    if Path::new(&args[1]).is_dir(){
        writeln!(std::io::stderr(),"second argument must be  a file").unwrap();
        std::process::exit(1);
    }
    let path: &Path = Path::new(&args[0]);
    match visite_dir(&path,&args[1]) {
        Ok(()) => {},
        Err(err) => panic!("{:?}",err),
    }
}

fn visite_dir(dir:&Path,filename:&str) ->io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)?{
            let entry  =  entry?;
            let path = entry.path();
            if path.is_dir() {
                visite_dir(&path,filename)?;
            }else if  path.is_file() {
                let path = path.to_str().unwrap();
                if let Some((_,"sql")) = parse_pair(&path,'.'){
                    read_file(&path,filename).unwrap();
                }
            }
        }
    }
    Ok(())
}

fn  read_file(path:&str,filename:&str)->io::Result<()>
{
    let f  = File::open(path).unwrap();
    let mut f = BufReader::new(f);
    let mut  buf = vec![];
    let _ = f.read_until(0,&mut buf).unwrap();
    write_file(filename,&buf)?;
    Ok(())
}

fn parse_pair(s:&str,separa:char)->Option<(&str,&str)>{
    match s.find(separa) {
        None => None,
        Some(index) =>{
            match (&s[..index],&s[index+1..]){
                (l,r) => Some((l,r)),
            }
        }
    }
}

fn write_file(filename:&str,streams:&[u8]) ->io::Result<()>{
    let file  = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(filename);
    match file {
        Ok(mut stream) => {
            stream.write_all(streams)?;
        }
        Err(err) => {
            println!("{:?}",err)
        }
    }
    Ok(())
}
