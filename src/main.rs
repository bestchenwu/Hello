extern crate iron;
//准备利用mime包导出来的宏
#[macro_use] extern crate mime;
//use 表示导入包中的公共属性
use iron::prelude::*;
use iron::status;
use iron::status::Status;

extern crate router;
use router::Router;
use urlencoded::UrlEncodedBody;

extern crate urlencoded;
fn main() {
    let mut router = Router::new();
    router.get("/",get_form,"root");
    router.post("add",add,"add");
    println!("Serving on http://localhost:8080");
    Iron::new(router).http("localhost:8080").unwrap();
}

fn get_form(_resquest: &mut Request) ->IronResult<Response>{
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

fn add(request:&mut Request)->IronResult<Response>{
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>(){
        Err(e)=>{
            response.set_mut(Status::BadRequest);
            response.set_mut(format!("error:{:?}",e));
            return Ok(response)
        },
        Ok(map)=>map
    };
    let unparsed_numbers = match form_data.get("n"){
        None=>{
            response.set_mut(Status::NoContent);
            response.set_mut("no input error");
            return Ok(response);
        },
        Some(nums)=>nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers{
        match u32::from_str(&unparsed) {
            Err(_)=>{
                response.set_mut(Status::NotAcceptable);
                response.set_mut(format!("bad input:{}",unparsed));
                return Ok(response);
            },
            Ok(n)=>{numbers.push(n);}
        }
    }
    Ok(response)
}