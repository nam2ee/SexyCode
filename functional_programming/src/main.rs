fn main() {

    let v = vec![1,2,3,4];
    let v = add(v,32);



}

fn add(v: Vec<i32>, target:i32 ) -> Vec<i32>{
    let mut v1 = v; 
    v1.push(target);
    v1 

    // 순수함수, + CoW 
}