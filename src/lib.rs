

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Error, FnArg, Ident, Type, ItemFn, Pat, PatIdent, ReturnType, 
    parse::{Parse, ParseStream}, punctuated::Punctuated, Token, Expr, ExprCall, ExprPath,
    visit_mut::{self, VisitMut}, Block,
};
use std::collections::HashSet;

struct KeyArgs {
    args: Vec<Ident>, 
}

impl Parse for KeyArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            args: args.into_iter().collect(),
        })
    }
}

// MauQueue 转换器
struct MauQueueTransformer;

impl VisitMut for MauQueueTransformer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        
        if let Expr::Call(call) = expr {
            if let Expr::Path(ExprPath { path, .. }) = &*call.func {
                if path.is_ident("MauQueue") {
                    if call.args.len() == 3 {
                        // 提取三个闭包参数
                        if let (
                            Expr::Closure(start_fn),
                            Expr::Closure(end_fn),
                            Expr::Closure(optimize_fn)
                        ) = (
                            &call.args[0],
                            &call.args[1],
                            &call.args[2]
                        ) {
                            // 创建循环代码
                            let new_block = quote! {{
                                let mut max = 0;
                                let start = (#start_fn)();
                                let end = (#end_fn)();
                                for i in start..=end {
                                    max = (#optimize_fn)(i);
                                }
                                max
                            }};

                            *expr = syn::parse2(new_block).unwrap();
                        }
                    }
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let _key_arg_names = parse_macro_input!(attr as KeyArgs).args;

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;

    // 无缓存版本的函数名: func_no_cache
    let no_cache_name = Ident::new(&format!("{}_no_cache", fn_name), fn_name.span());

    // 哈希表的名字 - 使用更安全的命名避免冲突
    let cache_name = Ident::new(&format!("{}_CACHE", fn_name.to_string().to_uppercase()), fn_name.span());

    // 提取参数和类型
    let (args, param_types): (Vec<_>, Vec<_>) = input_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let ident = match &*pat_type.pat {
                    Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
                    _ => {
                        return Err(Error::new(
                            pat_type.span(),
                            "only simple identifiers are supported",
                        ))
                    }
                };
                let ty = &*pat_type.ty;
                Ok((ident, ty))
            }
            _ => Err(Error::new(arg.span(), "self parameters are not supported")),
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .unzip();

    // 提取参数中的不可变引用
    let mut immutable_references = HashSet::<String>::new();
    for arg in input_fn.sig.inputs.iter() {
        if let FnArg::Typed(pat_type) = arg {
            if let Type::Reference(ty_ref) = &*pat_type.ty {
                if ty_ref.mutability.is_none() {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        immutable_references.insert(pat_ident.ident.to_string());
                    }
                } else {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        return Error::new(ty_ref.span(), format!("memo supports only immutable references in parameters, but {} is mutable", pat_ident.ident))
                            .to_compile_error()
                            .into();
                    }
                }
            }
        }
    }

    if args.is_empty() { return quote! {#fn_vis #fn_name(#fn_inputs) #fn_output #fn_block}.into(); }

    let (key_args, key_types): (Vec<_>, Vec<_>) = args.iter()
        .zip(param_types.into_iter())
        .filter(|(arg, _)| !immutable_references.contains(&arg.to_string()))
        .map(|(arg, ty)| (arg.clone(), ty.clone()))
        .unzip();   

    let key_type = if key_types.len() == 1 {
        quote! { #(#key_types)* }
    } else {
        quote! { (#(#key_types),*) }
    };

    let key_exprs = key_args.iter().map(|arg| quote! { #arg.clone() });
    let key_tuple = quote! { (#(#key_exprs),*) };

    let call_args = args.iter().map(|arg| quote! { #arg });

    let return_type = match fn_output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    // 转换函数体中的 MauQueue 调用
    let mut transformed_block = fn_block.clone();
    let mut transformer = MauQueueTransformer;
    transformer.visit_block_mut(&mut transformed_block);

    let create_cache = quote! {
        static #cache_name: ::std::sync::LazyLock<::std::sync::Mutex<::std::collections::HashMap<#key_type, #return_type>>> = ::std::sync::LazyLock::new(|| {
            ::std::sync::Mutex::new(::std::collections::HashMap::new())
        });
    };

    let no_cache_fn = quote! {
        #fn_vis fn #no_cache_name(#fn_inputs) #fn_output #transformed_block
    };

    // 重新定义原始函数，添加缓存功能
    let expanded = quote! {
        #create_cache
        #no_cache_fn
        
        // 重新定义原始函数名，添加缓存逻辑
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            let key = #key_tuple;
            // 检查缓存
            {   
                let cache = #cache_name.lock().unwrap();
                if let Some(result) = cache.get(&key) {
                    return result.clone();
                }
            }
            // 计算并缓存结果
            let result = #no_cache_name(#(#call_args),*);
            let mut cache = #cache_name.lock().unwrap();
            cache.insert(key, result.clone());
            result
        }
    };

    expanded.into()
}