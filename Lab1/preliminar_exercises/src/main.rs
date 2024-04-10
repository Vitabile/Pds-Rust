use std::time::SystemTime;
use std::fs;

const PATH: &str = "src/test.txt";
const PATH2: &str = "src/test2.txt";
const PATH3: &str = "src/a.txt";

fn main() {
    println!("Letture con \x1b[30mread_to_string\x1b[0m:");
    let str = read_to_string(PATH);
    let str2 = read_to_string(PATH2);
    read_to_string(PATH3);
    
    println!("\nLettura con \x1b[30mread\x1b[0m:");
    read(PATH, &str);
    read(PATH2, &str2);

    println!("\nScrittura x10 ...");
    write(PATH, str.as_str());

    println!("\nPrint del Nodo:");
    //costruisco 3 struct, la penalità e l'inefficienza è data dal fatto che ogni volta va distrutto e creato un nuovo oggetto e rilasciata la sua memoria
    let mut node = (Node::new("nodo".to_string())).size(10).count(5);
    println!("{}",node.to_string());
    node.grow();
    node.inc();
    println!("{}",node.to_string());

    
}

fn write(path: &str, str: &str){
    let res = fs::write(PATH, str.repeat(10));
    let time = SystemTime::now();
    match res {
        Err(_) => {
            print_error(Error::Simple(time));
            print_error(Error::Complex(time, format!("Errore in scrittura: \x1b[31m{}\x1b[0m non trovato!",path)));
        }
        Ok(_) => println!("Scrittura eseguita!")
    }
}

fn read_to_string(path: &str) -> String{
    let content = fs::read_to_string(path);
    let time = SystemTime::now();
    if content.is_ok(){
        let string = content.unwrap();
        println!("Lettura eseguita: \x1b[32m{string}\x1b[0m");
        string
    }else{
        print_error(Error::Simple(time));
        print_error(Error::Complex(time,format!("path \x1b[31m{}\x1b[0m non trovato!",path)));
        "Errore".to_string()
    }
}

fn read(path: &str, str: &String){
    let content = fs::read(path);
    if content.is_ok(){
        let string = content.unwrap();
        for c in str.chars(){
            print!("{c}  ");
        }
        print!("\n");
        for b in string{
            print!("{:02x} ",b);
        }
        print!("\n")
    }else{
        println!("Errore in lettura: path \x1b[31m{path}\x1b[0m non trovato!");
    }

}

enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String),
}

fn print_error(e: Error){
    let now = SystemTime::now();
    match e {
        Error::Simple(s) => {
            let duration = now.duration_since(s);
            match duration{
                Ok(d) => println!("Simple error: Detected {} ns ago",d.as_nanos()),
                Err(e) =>  println!("{:?}",e),
            }
        }
        Error::Complex(s2,str) => {
            let duration = now.duration_since(s2);
            match duration{
                Ok(d) => println!("Complex error: {} has been detected {} ns ago",str, d.as_nanos()),
                Err(e) => println!("{:?}",e),
            }
        }
    }
}

struct Node{
    name: String,
    size: u32,
    count: u32,
}
impl Node{
    pub fn new(name: String) -> Node{
        Node{name, size:0, count:0}
    }

    pub fn size(self, s: u32) -> Self{
        Self{size:s, .. self}
    }
    pub fn count(self, c: u32) -> Self{
        Self{count:c, .. self}
    }
    pub fn to_string(&self) -> String{
        format!("name:{} size:{} count:{}",self.name, self.size, self.count)
    }
    pub fn grow(&mut self){
        self.size += 1;
    }
    pub fn inc(&mut self){
        self.count += 1;
    }
}