#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}


fn main() {
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik and Carol Nichols");

    let book;
    {
        let short_title = &title[..4];
        book = Book {
            title: short_title,
            author: &author,
        };
    } // short_title의 라이프타임이 여기서 끝남

    println!("{} by {}", book.title, book.author); // 유효하지 않은 참조 아님! 
                                                    // 데이터 자체는 title에 있으므로 유효함


    // 그래서 재귀적으로 되어있어서
    // 명시적으로 라이프타임을 달지 않으면 그런 검사를 못하는데,
    // 명시적으로 달아준다면 로직이 생겨서 검사할 수 있다 ! 

    /* 이제 문제가 되는 예시를 알아보자!  */

    let book;

    {

        let title = String::from("Hello");
        let title2 = String::from("World");

        
        book = Book {
            title: &title,
            author: &title2,
        };
        
    }
    //println!("{:?}", book);
}


