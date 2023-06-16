  use proc_macro::{TokenStream};
use syn::{parse_macro_input, ItemFn};
use quote::{quote};

#[proc_macro_attribute]
pub fn has_permit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
    let func_vis = &func.vis; // pub
    let func_block = &func.block;//.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    // 提取参数，参数可能是多个
    // let params: Vec<_> = func_inputs.iter().map(|i| {
    //     println!("{:?}",i);
    //     match i {
    //         // 提取形参的pattern
    //         // https://docs.rs/syn/1.0.1/syn/struct.PatType.html
    //         FnArg::Typed(ref val) => {
    //            ( ( &val).pat.to_token_stream().to_string(),
    //            ( &val).ty.to_token_stream().to_string())
    //         } // pat没有办法移出val，只能借用，或者val.pat.clone()
    //         _ => unreachable!("it's not gonna happen."),
    //     }
    // }).collect();
  //  println!("{:?}",func_inputs);
//println!("{:?}", params);
    // 解析attr
    //let attr = parse_macro_input!(attr as AttributeArgs);
    // let i=  attr.get(0);
    // for n in attr{
    //     match n. {
    //         Meta(m)=>{m;},
    //         Lit(l)=>{l;}
    //     }
    // }
    let s = attr.to_string();
    //println!("aaaa{}", attr.to_string());

    // // 提取attr的ident
    // let attr_ident = match attr.get(0).as_ref().unwrap() {
    //     NestedMeta::Meta(Meta::Path(ref attr_ident)) => attr_ident.clone(),
    //     _ => unreachable!("it not gonna happen."),
    // };

    let expanded = quote! { // 重新构建函数执行
        #func_vis #func_asyncness fn #func_name #func_generics(req_in_permit:HttpRequest,#func_inputs) #func_output{
            match crate::token_auth::check_permit(req_in_permit, #s).await {//fixme 判断参数中是否存在httpRequest，以后再说
                 None =>  #func_block
             Some(res) => { return res.resp_json(); }
            }

        }
    };
//    Typed(PatType { attrs: [], pat: Ident(PatIdent { attrs: [], by_ref: None, mutability: None, ident: Ident { ident: "req", span: #0 bytes(18869..18872) }, subpat: None }), colon_token: Colon, ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "HttpRequest", span: #0 bytes(18874..18885) }, arguments: None }] } }) })

    //println!("{}", expanded.to_string());
    expanded.into()
}
