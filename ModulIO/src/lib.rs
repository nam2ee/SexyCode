use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


pub fn run(config: &Config) -> Result<(), Box<dyn Error> > {

    let mut f = File::open(&config.filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())

}

/*

struct Config<'a> {
    query: &'a String,
    filename: &'a String
}
- 이렇게 구현할 수도 있지만, 조금 더 깔끔하게 하기 위해서
참조가 아니라 아예 데이터를 복사하는 (깊은복사) clone을 쓴다 
*/
#[derive(Debug)]
pub struct Config{
    pub query: String,
    pub filename: String
}

impl Config{

    #[warn(dead_code)]
    pub fn parse_config(list: &[String]) -> Config{
    let query = list[1].clone();
    // &list[1].clone()을 하면 list[1]이 아니라
    // 참조를 복사하므로 안된다! 
    let filename = list[2].clone();

    let result = Config{
        query,filename,
    };
    result
    }

    pub fn new(list: &[String]) -> Result<Config,&str> {
    
    if list.len()<3
    {
        return Err("not enough args!");
        // 문자열 리터럴은 'static이다! 
    }

    let query = list[1].clone();
    // &list[1].clone()을 하면 list[1]이 아니라
    // 참조를 복사하므로 안된다! 
    let filename = list[2].clone();

    Ok(Config{query,filename})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
