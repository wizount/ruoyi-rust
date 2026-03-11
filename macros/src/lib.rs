mod trans;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::Parser;
use syn::Data::Struct;
use syn::{parse_macro_input, parse_quote, DataStruct, DeriveInput, Field, FnArg, ItemFn, LitFloat, LitStr, Meta};

/// 权限验证宏
///
/// # 功能
/// 该宏用于在函数执行前进行权限验证，只有验证通过才会执行原函数
///
/// # 使用示例
/// ```rust
/// #[pre_authorize("system:user:list")]
/// pub async fn list_users() -> Response {
///     // 业务逻辑
/// }
///
/// #[pre_authorize("system:user:add", user_info)]
/// pub async fn add_user() -> Response {
///     // 业务逻辑
/// }
/// ```
///
/// # 参数说明
/// - 第一个参数（可选）：权限字符串，如 "system:user:list"，用于检查用户是否具有该权限
/// - 第二个参数（可选）：用户缓存变量名，默认为 `user_cache`，指定从哪个变量获取用户信息
///
/// # 宏展开逻辑
/// 1. 提取原函数的所有属性、声明等信息
/// 2. 解析宏属性参数，获取权限字符串和用户标识符
/// 3. 如果未提供用户标识符，默认使用 `user_cache`
/// 4. 如果未提供权限字符串，默认使用空字符串
/// 5. 重新构建函数签名，添加 `UserCache` 参数
/// 6. 在函数体中添加权限检查逻辑
/// 7. 权限验证通过（返回 None）则执行原函数体
/// 8. 权限验证失败（返回 Some(res)）则直接返回错误响应
///
/// # 权限检查
/// 通过调用 `crate::web::check_permit` 函数进行权限验证
/// - 返回 `None` 表示验证通过，执行原函数
/// - 返回 `Some(res)` 表示验证失败，直接返回错误响应
#[proc_macro_attribute]
pub fn pre_authorize(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析传入的函数项，获取函数的所有信息
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
    let func_vis = &func.vis; // 函数可见性（如 pub、pub(crate) 等）
    let func_block = &func.block; // 函数体，包含函数的实际实现代码块

    let func_decl = &func.sig; // 函数签名，包含函数名、参数、返回值等信息

    let func_attrs = &func.attrs; // 函数上的其他属性（如 #[doc]、#[async_trait] 等）
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 是否为异步函数（async fn）
    let func_generics = &func_decl.generics; // 函数泛型参数
    let func_inputs = &func_decl.inputs; // 函数输入参数列表
    let func_output = &func_decl.output; // 函数返回类型

    // 定义属性解析器，用于解析宏属性参数
    let parser = |input: syn::parse::ParseStream| {
        let mut permit_str = None; // 权限字符串，用于检查用户是否具有该权限
        let mut user_ident = None; // 用户缓存标识符，指定从哪个变量获取用户信息

        while !input.is_empty() {
            if input.peek(syn::LitStr) {
                // 解析权限字符串，如 "system:user:list"
                permit_str = Some(input.parse::<syn::LitStr>()?);
            } else if input.peek(syn::Ident) {
                // 解析用户标识符，如 user_info、current_user 等
                user_ident = Some(input.parse::<syn::Ident>()?);
            } else if input.peek(syn::Token![,]) {
                // 跳过逗号分隔符
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok((permit_str, user_ident))
    };

    // 解析宏属性参数，获取权限字符串和用户标识符
    let (mut permit_str, mut user_ident) = match parser.parse(attr) {
        Ok(attr) => attr,
        Err(e) => return e.to_compile_error().into(), // 解析失败时返回编译错误
    };

    // 如果没有提供用户标识符，使用默认值 `user_cache`
    if user_ident.is_none() {
        user_ident = Some(parse_quote!(user_cache));
    }
    // 如果没有提供权限字符串，使用默认空字符串
    if permit_str.is_none() {
        permit_str = Some(parse_quote!(""));
    }

    // 构建展开后的代码，包含权限检查逻辑
    let expanded = quote! { // 重新构建函数执行
        // 保留原函数的所有属性
        #(#func_attrs)*
        // 重新构建函数签名：
        // - 添加 UserCache 类型的参数作为第一个参数
        // - 保留原有的可见性、异步性、函数名、泛型、参数和返回值
        #func_vis #func_asyncness fn #func_name #func_generics(#user_ident: crate::UserCache,#func_inputs) #func_output{
            // 在函数体中添加权限检查逻辑
            // 调用 crate::web::check_permit 进行权限验证
            match crate::web::check_permit(&#user_ident, #permit_str).await {
                // 权限验证通过（None），执行原函数体
                None =>  #func_block
                // 权限验证失败（Some(res)），直接返回错误响应
                Some(res) => { res.into_response() }
            }
        }
    };

    // 返回展开后的代码
    expanded.into()
}

//为查询DTO增加page_no和page_size，并提供impl From<&#ident> for rbatis::PageRequest
//根据参数增加params
//建议放在struct首位
#[proc_macro_attribute]
pub fn page_request(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut is_params = false;
    let mut is_data_scope = false;
    let mut no_page = false;

    //https://docs.rs/syn/latest/syn/macro.parse_macro_input.html#usage-with-parser
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("params") {
            is_params = true;
            Ok(())
        } else if meta.path.is_ident("dataScope") {
            is_data_scope = true;
            Ok(())
        } else if meta.path.is_ident("noPage") {
            no_page = true;
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });
    parse_macro_input!(attr with parser);
    let params = if is_params {
        quote! {
            pub params:Option<std::collections::HashMap<String, String>>,
        }
    } else {
        quote! {}
    };

    // 将输入的 token 流解析为 `DeriveInput`
    let original_struct = parse_macro_input!(input as DeriveInput);

    // 从输入中解构出 data 和 ident 字段
    let DeriveInput {
        data,
        ident,
        attrs: atrrs_s,
        ..
    } = original_struct.clone();

    if let Struct(data_struct) = data {
        // 从这个数据结构中提取字段
        let DataStruct { fields, .. } = data_struct;
        // 创建用于作为输出的变量 new_fields
        let mut new_fields = quote!();
        for Field {
            // 该字段的标识符
            ident,
            // 该字段的属性
            attrs,
            // 该字段的可见性
            vis,
            // 分隔符 `:`
            colon_token,
            // 该字段的类型
            ty,
            ..
        } in fields
        {
            new_fields.extend(quote! {
                #(#attrs)*
                #vis #ident #colon_token #ty,
            });
        }
        let mut data_scope_impl = quote! {};
        if is_data_scope {
            data_scope_impl = quote! {
              impl crate::DataScopeTrait for #ident {
                 fn clear_data_scope_params(&mut self) {
                     let mut params =self.params.clone().unwrap_or_default();
                     params.remove("dataScope");
                     self.params=params.into();
                 }
                 fn set_data_scope_params(&mut self,value:&str) {
                     let mut params =self.params.clone().unwrap_or_default();
                     params.insert("dataScope".to_string(),value.to_string());
                     self.params=params.into();
                 }
             }
            }
        }

        let struct_page_extend = if no_page {
            quote! {}
        } else {
            quote! {
                #[serde(rename(deserialize = "pageNum"))]
                #[validate(range(min = 1))]
                pub page_no: Option<u64>,
                #[validate(range(min = 1, max = 50))]
                pub page_size: Option<u64>,
            }
        };
        let page_impl_extend = if no_page {
            quote! {}
        } else {
            quote! {
               impl From<&#ident> for rbatis::PageRequest {
                fn from(arg: &#ident) -> Self {
                rbatis::PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
                }
            }

            }
        };
        let res = quote! {
        #(#atrrs_s)*
        #[derive(validator::Validate)]
        pub struct #ident {
            #struct_page_extend
            #new_fields
            #params
        }
            #page_impl_extend
         #data_scope_impl
            };
        res.into()
    } else {
        // 如果目标不是命名结构，则触发 panic 错误
        panic!("DeriveCustomModel 只能用于命名结构")
    }
}

//不打算使用，实际情况太复杂
// 根据注释情况生成DTO
#[proc_macro_attribute]
pub fn gen_dto(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut add_additional: Option<LitStr> = None;
    let mut update_additional: Option<LitStr> = None;

    //https://docs.rs/syn/latest/syn/macro.parse_macro_input.html#usage-with-parser
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("addAdditional") {
            add_additional = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("updateAdditional") {
            update_additional = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });

    parse_macro_input!(attr with parser);

    // 将输入的 token 流解析为 `DeriveInput`
    let original_struct = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        data,
        ident,
        attrs: atrrs_s,
        ..
    } = original_struct.clone();
    let struct_name = &ident;
    //重新生成源的struct
    let mut struct_expand = quote! {};
    let add_dto_name = quote::format_ident!("{}AddDTO", struct_name);
    let update_dto_name = quote::format_ident!("{}UpdateDTO", struct_name);
    let vo_name = quote::format_ident!("{}VO", struct_name);
    let mut add_dto_expand = quote! {};
    let mut update_dto_expand = quote! {};
    let mut vo_expand = quote! {};
    let mut add_from_expand = quote! {};
    let mut update_from_expand = quote! {};
    let mut vo_from_expand = quote! {};

    if let Struct(data_struct) = data {
        // 从这个数据结构中提取字段
        let DataStruct { fields, .. } = data_struct;
        // 创建用于作为输出的变量 new_fields
        // let mut new_fields = quote!();
        for Field {
            // 该字段的标识符
            ident,
            // 该字段的属性
            attrs,
            // 该字段的可见性
            vis,
            // 分隔符 `:`
            // colon_token,
            // 该字段的类型
            ty,
            ..
        } in fields
        {
            let mut validate_expand = quote! {};
            let mut serde_attr_expand = quote! {};
            let mut has_add = false;
            let mut has_update = false;
            let mut has_vo = false;
            for attr in attrs {
                if attr.path().is_ident("validate") {
                    validate_expand.extend(quote! {#attr});
                } else if attr.path().is_ident("serde") {
                    serde_attr_expand.extend(quote! {#attr});
                } else if attr.path().is_ident("dto") {
                    match attr.meta {
                        Meta::List(e) => {
                            //fixme 更改
                            let s = e.tokens.to_string();
                            let ss = s.split(",").map(|s| s.trim()).collect::<Vec<_>>();
                            if ss.contains(&"add") {
                                has_add = true;
                            }
                            if ss.contains(&"update") {
                                has_update = true;
                            }
                            if ss.contains(&"vo") {
                                has_vo = true;
                            }
                        }
                        _ => {}
                    }
                } else {
                    struct_expand.extend(quote! {
                        #attr
                    })
                }
            }
            struct_expand.extend(quote! {
                #vis #ident: #ty,
            });
            if has_add {
                add_dto_expand.extend(quote! {
                    #validate_expand
                    #vis #ident: #ty,
                });
                add_from_expand.extend(quote! {
                    #ident:arg.#ident,
                })
            } else {
                add_from_expand.extend(quote! {
                    #ident:None,
                })
            }
            if has_update {
                update_dto_expand.extend(quote! {
                    #validate_expand
                    #vis #ident: #ty,
                });
                update_from_expand.extend(quote! {
                    #ident:arg.#ident,
                })
            } else {
                update_from_expand.extend(quote! {
                    #ident:None,
                })
            }
            if has_vo {
                vo_expand.extend(quote! {
                    #serde_attr_expand
                    #vis #ident: #ty,
                });
                vo_from_expand.extend(quote! {
                    #ident:arg.#ident,
                })
            }
        }

        if add_additional.is_some() {
            let a = add_additional.unwrap().to_token_stream().to_string();
            let a = a[1..a.len() - 1].to_string();
            let a = a.parse::<proc_macro2::TokenStream>().unwrap();
            add_dto_expand.extend(a);
        }

        let expanded = quote! {
             #(#atrrs_s)*
             pub struct #struct_name {
                #struct_expand
            }
            #[derive(serde::Serialize, serde::Deserialize, validator::Validate, Clone, Debug)]
            #[serde(rename_all = "camelCase")]
            pub struct #add_dto_name {
                #add_dto_expand
            }
            impl From<#add_dto_name> for #struct_name {
                fn from(arg: #add_dto_name) -> Self {
                    #struct_name {
                        #add_from_expand
                    }
                }
            }
            #[derive(serde::Serialize, serde::Deserialize, validator::Validate, Clone, Debug)]
            #[serde(rename_all = "camelCase")]
            pub struct #update_dto_name {
                #update_dto_expand
            }
            impl From<#update_dto_name> for #struct_name {
                fn from(arg: #update_dto_name) -> Self {
                    #struct_name {
                        #update_from_expand
                    }
                }
            }
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #vo_name {
                #vo_expand
            }

            impl From<#struct_name> for #vo_name {
                fn from(arg: #struct_name) -> Self {
                    Self {
                        #vo_from_expand
                    }
                }
            }
        };
        expanded.into()
    } else {
        // 如果目标不是命名结构，则触发 panic 错误
        panic!("DeriveCustomModel 只能用于命名结构")
    }
}

///同ruoyi DataScope
#[proc_macro_attribute]
pub fn data_scope(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
    let func_vis = &func.vis; // pub

    //去掉大括号
    let mut stmts_expanded = quote! {};
    func.block
        .stmts
        .iter()
        .for_each(|r| stmts_expanded.extend(r.to_token_stream()));
    let func_decl = &func.sig; // 函数申明

    let func_attrs = &func.attrs;
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    let parser = |input: syn::parse::ParseStream| {
        let mut dept_alias = Some(parse_quote!(""));
        let mut user_alias = Some(parse_quote!(""));
        let mut user_cache_ident = Some(parse_quote!(user_cache));

        while !input.is_empty() {
            if input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                if input.peek(syn::Token![=]) {
                    input.parse::<syn::Token![=]>()?;
                    match ident.to_string().as_str() {
                        "deptAlias" => {
                            if input.peek(syn::LitStr) {
                                dept_alias = Some(input.parse::<syn::LitStr>()?);
                            }
                        }
                        "userAlias" => {
                            if input.peek(syn::LitStr) {
                                user_alias = Some(input.parse::<syn::LitStr>()?);
                            }
                        }
                        _ => {}
                    }
                } else {
                    user_cache_ident = Some(ident);
                }
            } else if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            } else {
            }
        }
        Ok((dept_alias, user_alias, user_cache_ident))
    };
    let (dept_alias, user_alias, user_cache_ident) = match parser.parse(attr) {
        Ok(attr) => attr,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut dto_ident = None;
    func_inputs.iter().for_each(|i| {
        match i {
            // 提取形参的pattern
            // https://docs.rs/syn/1.0.1/syn/struct.PatType.html
            FnArg::Typed(ref val) => {
                let ty = (&val).ty.to_token_stream().to_string();
                if ty.ends_with("DTO") {
                    dto_ident = Some((&val).pat.to_token_stream());
                }
            } // pat没有办法移出val，只能借用，或者val.pat.clone()
            _ => {}
        }
    });
    let dept_alias = dept_alias.map(|a| a.to_token_stream()).unwrap_or_default();
    let expanded = quote! { // 重新构建函数执行
        #(#func_attrs)*
        #func_vis #func_asyncness fn #func_name #func_generics(#func_inputs,#user_cache_ident:&crate::UserCache) #func_output{
            let mut #dto_ident = #dto_ident.clone();
            crate::web::data_scope::build_data_scope(&mut #dto_ident, #dept_alias, #user_alias,#user_cache_ident).await?;
            #stmts_expanded

        }
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn transactional(ident: TokenStream, item: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(ident as Ident);
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
    let func_vis = &func.vis; // pub
    let func_block = &func.block; //.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明

    let func_attrs = &func.attrs;

    let mut has_pool_st =trans::HasPoolSt{};
    syn::visit::visit_block(&mut has_pool_st, & func_block);
    let expanded = quote! { // 重新构建函数执行
        #(#func_attrs)*
        #func_vis #func_decl{
            let #ident = crate::pool!().acquire_begin().await?;
            let func= async || #func_block;
             let res = func().await;
            match res {
                Ok(_)=>{
                    #ident.commit().await?;}
                Err(_)=>{
                    #ident.rollback().await?;}
            }
            res
        }
    };
    expanded.into()
}
#[derive(Debug)]
struct ExcelAttribute {
    name: Option<LitStr>,
    dict_type: Option<LitStr>,
    default_value: Option<LitStr>,
    read_converter_exp: Option<LitStr>,
    num_format: Option<LitStr>,
    width: Option<LitFloat>,
    attr_type: Option<syn::Path>,
}
macro_rules! to_token_string {
    ($self_:ident,$name:ident,$tokens:ident) => {
        match &$self_.$name {
            None => $tokens.extend(quote! {
                $name:None,
            }),
            Some(t) => $tokens.extend(quote! {
                $name:Some(#t.to_string()),
            }),
        }
    };
}
macro_rules! to_token_int {
    ($self_:ident,$name:ident,$tokens:ident) => {
        match &$self_.$name {
            None => $tokens.extend(quote! {
                $name:None,
            }),
            Some(t) => $tokens.extend(quote! {
                $name:Some(#t),
            }),
        }
    };
}
impl ToTokens for ExcelAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.name {
            None => tokens.extend(quote! {
                name:"".to_string(),
            }),
            Some(t) => tokens.extend(quote! {
                name:#t.to_string(),
            }),
        }
        to_token_string!(self, dict_type, tokens);
        to_token_string!(self, default_value, tokens);
        to_token_string!(self, read_converter_exp, tokens);
        to_token_string!(self, num_format, tokens);
        to_token_int!(self, width, tokens);
        to_token_int!(self, attr_type, tokens);
    }
}
#[proc_macro_derive(Export, attributes(excel))]
pub fn export(item: TokenStream) -> TokenStream {
    // 将输入的 token 流解析为 `DeriveInput`
    let original_struct = syn::parse_macro_input!(item as syn::DeriveInput);
    let DeriveInput { data, ident, .. } = original_struct.clone();
    let mut expand = quote! {};

    let parser = |input: syn::parse::ParseStream| {
        let mut excel_attr = ExcelAttribute {
            name: None,
            dict_type: None,
            default_value: None,
            read_converter_exp: None,
            num_format: None,
            width: None,
            attr_type: None,
        };
        // 解析位置参数（字符串字面量）
        if input.peek(syn::LitStr) {
            excel_attr.name = Some(input.parse::<syn::LitStr>()?);
        }

        // 解析逗号分隔的参数
        while !input.is_empty() {
            input.parse::<syn::Token![,]>()?;

            // 解析命名参数
            if input.peek(syn::Ident) && input.peek2(syn::Token![=]) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                match ident.to_string().as_str() {
                    "name" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.name = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "dictType" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.dict_type = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "defaultValue" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.default_value = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "readConverterExp" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.read_converter_exp = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "numFormat" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.num_format = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "width" => {
                        if input.peek(syn::LitFloat) {
                            excel_attr.width = Some(input.parse::<syn::LitFloat>()?);
                        }
                    }
                    "attrType" => {
                        excel_attr.attr_type = Some(input.parse::<syn::Path>()?);
                    }
                    _ => {
                        return Err(input.error(format!(
                            "Unknown parameter {}, expected `path`",
                            ident.to_string().as_str()
                        )))
                    }
                }
            } else if excel_attr.name.is_none() && input.peek(syn::LitStr) {
                // 处理没有前置逗号的位置参数
                excel_attr.name = Some(input.parse::<syn::LitStr>()?);
            } else {
                return Err(input.error("Expected named parameter or string literal"));
            }
        }
        Ok(excel_attr)
    };

    if let Struct(data_struct) = data {
        // 从这个数据结构中提取字段
        let DataStruct { fields, .. } = data_struct;
        // 创建用于作为输出的变量 new_fields
        // let mut new_fields = quote!();
        for Field {
            // 该字段的标识符
            ident,
            // 该字段的属性
            attrs,
            ..
        } in fields
        {
            for attr in attrs {
                if attr.path().is_ident("excel") {
                    let dto_attr = attr.parse_args_with(parser).unwrap();
                    let ident_camel_case = to_camel_case(&(ident.clone().map(|s| s.to_string()).unwrap_or_default()));
                    expand.extend(quote! {
                        excel_gen_attr.push(crate::ExcelGenAttr{
                                camel_case_indent: #ident_camel_case.to_string(),
                               #dto_attr
                        });
                    })
                }
            }
        }
    }
    let res = quote! {
       impl crate::ExcelGenAttrTrait for #ident {
         fn get_excel_attr()->Vec<crate::ExcelGenAttr> {
                let mut excel_gen_attr=vec!();
                #expand
               excel_gen_attr
            }
        }
    };
    res.into()
}

use regex::Regex;

fn to_camel_case(text: &str) -> String {
    Regex::new(r"[_-]")
        .unwrap()
        .split(text)
        .enumerate()
        .map(|(i, x)| {
            if i != 0usize {
                let mut s = x.to_string();
                if let Some(r) = s.get_mut(0..1) {
                    r.make_ascii_uppercase()
                }
                s
            } else {
                x.to_string()
            }
        })
        .collect()
}

//将pool()!替换为tx_expr,默认不保留原先，
#[proc_macro_attribute]
pub fn replace_pool(attr: TokenStream, item: TokenStream) -> TokenStream {

    let parser = |input: syn::parse::ParseStream| {
        let mut keep_old = None;
        let mut tx_ident = None;

        while !input.is_empty() {
            if input.peek(syn::LitBool) {
                // 解析权限字符串
                keep_old = Some(input.parse::<syn::LitBool>()?);
            } else if input.peek(syn::Ident) {
                tx_ident = Some(input.parse::<syn::Ident>()?);
            } else if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok((keep_old, tx_ident))
    };
    let ( keep_old, tx_ident) = match parser.parse(attr) {
        Ok(attr) => attr,
        Err(e) => return e.to_compile_error().into(),
    };
    let keep_old= keep_old.is_some_and(|b|b.value);
    // 添加 tx 参数
    // let tx_param: FnArg = syn::parse_quote! {
    //     #ident: &rbatis::RBatis
    // };
    // input_fn.sig.inputs.push( tx_param);
    // 创建 AST 遍历器
    let tx_ident = tx_ident.unwrap_or(parse_quote!(tx));




    let old_fun=if keep_old {item.clone().into()}else{quote! { }};

    let mut visitor = crate::trans::ReplacePoolSt {
        tx: tx_ident.clone(),
    };


    let mut input_fn = parse_macro_input!(item as ItemFn);

    // 遍历并替换函数体中的所有pool()!表达式
    syn::visit_mut::visit_block_mut(&mut visitor, &mut input_fn.block);

    let func_vis = &input_fn.vis; // pub
    let func_block = &input_fn.block; //.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}

    let func_decl = &input_fn.sig; // 函数申明

    //  let func_attrs = &input_fn.attrs;
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output;


    let tx_func_name = quote::format_ident!("{}_tx", func_name);

    // let mut arg_names = Vec::new();
    //
    // for arg in func_inputs.iter() {
    //     match arg {
    //         syn::FnArg::Typed(pat_type) => {
    //             if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
    //                 arg_names.push(&pat_ident.ident);
    //             }
    //         }
    //         syn::FnArg::Receiver(_) => {
    //         }
    //     }
    // }


    let output=quote! {

        #old_fun
        #func_vis #func_asyncness fn #tx_func_name #func_generics(#func_inputs,#tx_ident:& rbatis :: executor::RBatisTxExecutor) #func_output
         #func_block

    };
    output.into()
}
