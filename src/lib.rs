use std::{
    collections::HashMap,
    io::{Error, Read, Write},
    net::TcpStream,
    fs,
};
#[allow(non_snake_case)]
// I dont get why people have like 5 files with 20 lines in it each (i dont have a repo that match the description trust me bro)
// Sorry if my code sucks. I dont even have the mental capacity of an 18 year old human
pub struct Reqdata {
    pub HttpVer: String,
    pub Method: String,
    pub Path: String,
    pub Header_fields: HashMap<String, String>,
    pub Body: String,
}
pub struct Httpreq {
    pub Reqdata:Reqdata,
    pub Conn: TcpStream,
}

pub struct Response{
    pub Status: String,
    pub HttpVer: String,
    pub Header: String,
    pub Body: Vec<u8>,
    pub Conn: TcpStream
}

impl Httpreq{
    // There is absolytely no need to use this function
    pub fn new_response(self) -> Response{
        return Response{Status:"".to_string(),HttpVer:"".to_string(),Header:"".to_string(),Body:Vec::new(),Conn:self.Conn};
    }
}

pub fn get_str_from_hashmap(h: HashMap<String,String>,k:&str) -> String{
    let a = match h.get(k).ok_or_else(||""){
        Ok(a) => {a},
        Err(n) => {n}
    };
    a.to_string()
}
// Load files into a vector
pub fn load_file(path:&str) -> Result<Vec<u8>,Error>{
    let mut file = match fs::File::open(path){
        Ok(fe) => {fe},
        Err(r) => {return Err(r)}
    };
    let mut buf:Vec<u8> = Vec::new();
    match file.read_to_end(&mut buf){
        Ok(_) => {return Ok(buf);},
        Err(e) => {return Err(e)}
    }
}

impl Response{ // Any errors? Not my problem
    // Usage: fn("HTTP/0.0 404 jfwsjgo")
    pub fn response_str(&mut self,ret:&str) -> std::io::Result<()>{
        match self.Conn.write(ret.as_bytes()){
            Ok(_) => {Ok(())},
            Err(e) => {
                return Err(e);
            } 
        }
    }
    pub fn insert_Header(&mut self,f:&str,v:&str){
        self.Header += format!("{}: {}\r\n",
                              f,v ).as_str()
    }
    // Return total sent bytes
    pub fn done(&mut self) -> std::io::Result<usize>{
        let u = match self.Conn.write(format!("{} {}\r\n{}\r\n",
                                                self.HttpVer,self.Status,self.Header).as_bytes()){ 
            Ok(s) => s,
            Err(a) => {return Err(a);}
        }; // I need 2 write calls
        let i = match self.Conn.write(self.Body.as_slice()){
            Ok(g) => g,
            Err(a) => {return  Err(a);}
        };
        return Ok(u);
    }
}

// Vec vec vec
pub fn parsereq(mut Conn: TcpStream,Body:bool) -> Result<Httpreq, Error> {
    let mut buffer:[u8; 1024] = [0; 1024]; // Idk if theres a way to handle that one funny guy that sends you 1MB of unsolicited data at a rapid rate of 1000 requests per second
    Conn.read(&mut buffer)?;
    let str: Vec<&str> = match std::str::from_utf8(&buffer) {
        Ok(ni) => ni.split("\n").collect(),
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)), // Not my problem now gtfo
    }; // I am new to rust if you dont mind
    let r = Reqdata {
        FormMap: HashMap::new(),
        HttpVer: "Unknown".to_owned(),
        Path: "Unknown".to_owned(),
        Method: "Unknown".to_owned(),
        Header_fields: HashMap::new(),
        Body: "".to_string(),
    };
    let mut st = Httpreq {Reqdata:r,Conn:Conn};
    let i = 0;
    for(_,s) /*dude wtf is &&str*/ in str.iter().enumerate(){
        if i == 0{
            let d:Vec<&str> = s.split(" ").collect();
            if d.len() != 3{
                continue;
            }
            st.Reqdata.Method = d[0].to_owned();
            st.Reqdata.Path = d[1].to_owned();
            st.Reqdata.HttpVer = d[2].to_owned();
            continue;
        }
        let replaced = s.replace(" ", ""); ///////////////////////////Rust borrow checker\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
        let f: Vec<&str> = replaced.split(":").collect();        
        if f.len() == 2 {
            st.Reqdata.Header_fields.insert(f[0].to_string(), f[1].to_string());
        }  
        if i + 2 >= str.len() && str[i+1] == "\r\n" && str[i+2] == "\r\n"{
            break;
        }
    }
    if Body{
        for _ in i..str.len(){
            st.Reqdata.Body += str[i]
        }
    }
    return Ok(st)
}
