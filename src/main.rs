extern crate iron;
//准备利用mime包导出来的宏
#[macro_use]
extern crate mime;

use std::str::FromStr;
//use 表示导入包中的公共属性
use iron::prelude::*;
use iron::status;
use iron::status::Status;

extern crate num;

use num::{Complex, ToPrimitive};

extern crate router;

use router::Router;
use urlencoded::UrlEncodedBody;

extern crate urlencoded;

fn main() {
    //利用rust开发的web服务器
    // let mut router = Router::new();
    // router.get("/",get_form,"root");
    // router.post("add",add,"add");
    // println!("Serving on http://localhost:8080");
    // Iron::new(router).http("localhost:8080").unwrap();
}

fn square_loop(c: Complex<f64>,limit:u32)->Option<u32> {
    let mut x = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit{
        x = x * x + c;
        if x.norm_sqr()>4.0{
            return Some(i);
        }
    }
    None
}

fn parse_pair<T:FromStr>(str:&str,seperaterChar:char)->Option<(T,T)>{
    match str.find(seperaterChar){
        None=>None,
        Some(index)=>{
            match (T::from_str(&str[0..index]),T::from_str(&str[index+1..])){
                (Ok(l),Ok(r))=>Some((l,r)),
                _=>None
            }
        }
    }
}

fn parse_complex(str:&str)->Option<Complex<f64>>{
    match parse_pair(str,',') {
        Some((l,r))=>Some(Complex{re:l,im:r}),
        _=>None
    }
}

#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i64>("12*34",'*'),Some((12,34)));

}

#[test]
fn test_parse_complex(){
    assert_eq!(parse_complex("12.5,-0.0625"),Some(Complex{re:12.5 as f64,im:-0.0625}));
}

fn get_form(_resquest: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html;Charset=Utf8));
    //r开头表示是原始字符串,后面的字符都无需转义
    response.set_mut(r#"
        <title>add calculator</title>
        <form action="/add" method="post">
            <input type="text" name ="n"/>
            <input type="text" name ="n"/>
            <button type="submit">submit add</button>
        </form>
    "#);
    Ok(response)
}

fn add(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(Status::BadRequest);
            response.set_mut(format!("error:{:?}", e));
            return Ok(response);
        }
        Ok(map) => map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(Status::NoContent);
            response.set_mut("no input error");
            return Ok(response);
        }
        Some(nums) => nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match &unparsed.parse::<u32>() {
            Err(_) => {
                response.set_mut(Status::NotAcceptable);
                response.set_mut(format!("bad input:{}", unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n.clone()); }
        }
    }
    let mut sum: u32 = 0;
    for k in &numbers {
        sum += k;
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html;Charset=Utf8));
    response.set_mut(format!("the numbers:{:?} sum is {}", numbers, sum));
    Ok(response)
}