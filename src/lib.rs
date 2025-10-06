

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Error, FnArg, Ident, Type, ItemFn, Pat, PatIdent, ReturnType, 
    parse::{Parse, ParseStream}, punctuated::Punctuated, Token, Expr, ExprPath, ExprRange,
    visit_mut::{self, VisitMut},
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

// 范围宏的解析结构
struct RangeMacro {
    closure: Expr,
    range: ExprRange,
}

impl Parse for RangeMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析闭包 |i| d[i]
        let closure = input.parse::<Expr>()?;
        
        // 解析逗号
        input.parse::<Token![,]>()?;
        
        // 解析中括号包围的范围表达式 [start..end]
        let content;
        syn::bracketed!(content in input);
        let range = content.parse::<ExprRange>()?;
        
        Ok(RangeMacro { closure, range })
    }
}

// 多参数宏的解析结构
struct MultiArgsMacro {
    args: Vec<Expr>,
}

impl Parse for MultiArgsMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();
        
        // 解析所有参数
        while !input.is_empty() {
            let arg = input.parse::<Expr>()?;
            args.push(arg);
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        
        Ok(MultiArgsMacro { args })
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

    // 对于引用参数，我们需要特殊处理
    // 对于引用类型，我们使用克隆的值作为缓存键，而不是地址
    let mut key_args = Vec::new();
    let mut key_types = Vec::new();
    let mut key_exprs = Vec::new();
    
    for (arg, ty) in args.iter().zip(param_types.into_iter()) {
        key_args.push(arg.clone());
        
        if immutable_references.contains(&arg.to_string()) {
            // 对于引用参数，使用克隆的值作为键
            // 检查是否是切片引用类型
            if let Type::Reference(ty_ref) = &*ty {
                if let Type::Slice(slice_ty) = &*ty_ref.elem {
                    // 对于 &[T] 类型，使用 Vec<T>
                    // 检查切片元素类型是否是 f64
                    if let Type::Path(type_path) = &*slice_ty.elem {
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "f64" {
                                // 对于 &[f64] 类型，使用 Vec<u64> 作为键
                                key_types.push(quote! { Vec<u64> });
                                key_exprs.push(quote! { #arg.iter().map(|x| x.to_bits()).collect() });
                            } else {
                                // 对于其他切片类型，使用 Vec<T> 作为键
                                let elem_ty = &*slice_ty.elem;
                                key_types.push(quote! { Vec<#elem_ty> });
                                key_exprs.push(quote! { #arg.to_vec() });
                            }
                        } else {
                            // 对于其他切片类型，使用 Vec<T> 作为键
                            let elem_ty = &*slice_ty.elem;
                            key_types.push(quote! { Vec<#elem_ty> });
                            key_exprs.push(quote! { #arg.to_vec() });
                        }
                    } else if let Type::Array(array_ty) = &*slice_ty.elem {
                        // 对于 &[[T; N]] 类型，检查内部数组元素类型
                        let inner_elem_ty = &*array_ty.elem;
                        if let Type::Path(type_path) = &*inner_elem_ty {
                            if let Some(ident) = type_path.path.get_ident() {
                                if ident == "f64" {
                                    // 对于 &[[f64; N]] 类型，使用 Vec<Vec<u64>> 作为键
                                    key_types.push(quote! { Vec<Vec<u64>> });
                                    key_exprs.push(quote! { #arg.iter().map(|row| row.iter().map(|x| x.to_bits()).collect()).collect() });
                                } else {
                                    // 对于其他嵌套数组类型，使用 Vec<Vec<T>> 作为键
                                    key_types.push(quote! { Vec<Vec<#inner_elem_ty>> });
                                    key_exprs.push(quote! { #arg.iter().map(|row| row.to_vec()).collect() });
                                }
                            } else {
                                key_types.push(quote! { Vec<Vec<#inner_elem_ty>> });
                                key_exprs.push(quote! { #arg.iter().map(|row| row.to_vec()).collect() });
                            }
                        } else {
                            key_types.push(quote! { Vec<Vec<#inner_elem_ty>> });
                            key_exprs.push(quote! { #arg.iter().map(|row| row.to_vec()).collect() });
                        }
                    } else {
                        // 对于其他切片类型，使用 Vec<T> 作为键
                        let elem_ty = &*slice_ty.elem;
                        key_types.push(quote! { Vec<#elem_ty> });
                        key_exprs.push(quote! { #arg.to_vec() });
                    }
                } else if let Type::Array(array_ty) = &*ty_ref.elem {
                    // 对于 &[T; N] 类型，使用 Vec<T> 作为键
                    let elem_ty = &*array_ty.elem;
                    // 检查是否是 f64 类型
                    if let Type::Path(type_path) = &*elem_ty {
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "f64" {
                                // 对于 &[f64; N] 类型，使用 Vec<u64> 作为键
                                key_types.push(quote! { Vec<u64> });
                                key_exprs.push(quote! { #arg.iter().map(|x| x.to_bits()).collect() });
                            } else {
                                // 对于其他数组类型，使用 Vec<T> 作为键
                                key_types.push(quote! { Vec<#elem_ty> });
                                key_exprs.push(quote! { #arg.to_vec() });
                            }
                        } else {
                            key_types.push(quote! { Vec<#elem_ty> });
                            key_exprs.push(quote! { #arg.to_vec() });
                        }
                    } else {
                        key_types.push(quote! { Vec<#elem_ty> });
                        key_exprs.push(quote! { #arg.to_vec() });
                    }
                } else {
                    // 对于其他引用类型，克隆引用指向的值
                    let elem_ty = &ty_ref.elem;
                    // 检查是否是 f64 类型，如果是则使用特殊处理
                    if let Type::Path(type_path) = &**elem_ty {
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "f64" {
                                // 对于 f64 类型，使用 u64 作为键（通过位模式转换）
                                key_types.push(quote! { u64 });
                                key_exprs.push(quote! { #arg.to_bits() });
                            } else {
                                key_types.push(quote! { #elem_ty });
                                key_exprs.push(quote! { #arg.clone() });
                            }
                        } else {
                            key_types.push(quote! { #elem_ty });
                            key_exprs.push(quote! { #arg.clone() });
                        }
                    } else {
                        key_types.push(quote! { #elem_ty });
                        key_exprs.push(quote! { #arg.clone() });
                    }
                }
            } else {
                // 非引用类型，直接使用
                key_types.push(quote! { #ty });
                key_exprs.push(quote! { #arg.clone() });
            }
        } else {
            // 对于非引用参数，直接使用
            key_types.push(quote! { #ty });
            key_exprs.push(quote! { #arg.clone() });
        }
    }

    let key_type = if key_types.len() == 1 {
        quote! { #(#key_types)* }
    } else {
        quote! { (#(#key_types),*) }
    };
    
    let key_tuple = quote! { (#(#key_exprs),*) };

    // 对于函数调用，我们直接使用原始参数名
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
            let cache_key = #key_tuple;
            // 检查缓存
            {   
                let cache = #cache_name.lock().unwrap();
                if let Some(result) = cache.get(&cache_key) {
                    return result.clone();
                }
            }
            // 计算并缓存结果
            let result = #no_cache_name(#(#call_args),*);
            let mut cache = #cache_name.lock().unwrap();
            cache.insert(cache_key, result.clone());
            result
        }
    };

    expanded.into()
}

/// min! 宏：在指定范围内找到最小值
/// 
/// 语法：min!(|i| d[i], start..end)
/// 
/// 示例：
/// ```rust
/// use mau::min;
///
/// let d = vec![3, 1, 4, 1, 5, 9];
/// let min_val = min!(|i| d[i], [0..d.len()]);
/// assert_eq!(min_val, 1);
/// ```
#[proc_macro]
pub fn min(input: TokenStream) -> TokenStream {
    // 首先尝试解析为范围语法（检查是否有闭包和范围）
    if let Ok(range_macro) = syn::parse::<RangeMacro>(input.clone()) {
        // 范围语法：min!(|i| d[i], [start..end])
        let closure = &range_macro.closure;
        let range = &range_macro.range;
        
        // 提取范围的开始和结束
        let start = range.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
        let end = range.end.as_ref().map(|e| quote! { #e });
        
        let expanded = if let Some(end_expr) = end {
            // 有结束范围的情况：start..end 或 start..=end
            let range_expr = match range.limits {
                syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
            };
            
            quote! {{
                let mut min_val = None;
                let mut min_index = 0;
                
                for __mau_idx in #range_expr {
                    let current_val = (#closure)(__mau_idx);
                    match min_val {
                        None => {
                            min_val = Some(current_val);
                            min_index = __mau_idx;
                        }
                        Some(min) => {
                            if current_val < min {
                                min_val = Some(current_val);
                                min_index = __mau_idx;
                            }
                        }
                    }
                }
                
                min_val.expect("Range cannot be empty")
            }}
        } else {
            // 无结束范围的情况：start..
            quote! {{
                let mut min_val = None;
                let mut min_index = 0;
                let mut __mau_idx = #start;
                
                loop {
                    let current_val = (#closure)(__mau_idx);
                    match min_val {
                        None => {
                            min_val = Some(current_val);
                            min_index = __mau_idx;
                        }
                        Some(min) => {
                            if current_val < min {
                                min_val = Some(current_val);
                                min_index = __mau_idx;
                            }
                        }
                    }
                    __mau_idx += 1;
                }
            }}
        };
        
        return expanded.into();
    }
    
    // 如果范围语法解析失败，尝试解析为多参数语法
    if let Ok(multi_args) = syn::parse::<MultiArgsMacro>(input) {
        // 多参数语法：min!(1, a, b, c, 3)
        if multi_args.args.is_empty() {
            return syn::Error::new(proc_macro2::Span::call_site(), "min! macro requires at least one argument").to_compile_error().into();
        }
        
        if multi_args.args.len() == 1 {
            // 只有一个参数，直接返回
            let arg = &multi_args.args[0];
            return quote! { #arg }.into();
        } else {
            // 多个参数，找到最小值
            let mut iter = multi_args.args.iter();
            let first = iter.next().unwrap();
            let rest = iter.collect::<Vec<_>>();
            
            return quote! {{
                let mut min_val = #first;
                #(
                    if #rest < min_val {
                        min_val = #rest;
                    }
                )*
                min_val
            }}.into();
        }
    }
    
    // 如果两种解析都失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), "Invalid syntax for min! macro. Use either min!(a, b, c) or min!(|i| expr, [start..end])").to_compile_error().into()
}

/// max! 宏：在指定范围内找到最大值
///
/// 语法：max!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::max;
///
/// let d = vec![3, 1, 4, 1, 5, 9];
/// let max_val = max!(|i| d[i], [0..d.len()]);
/// assert_eq!(max_val, 9);
/// ```
#[proc_macro]
pub fn max(input: TokenStream) -> TokenStream {
    // 首先尝试解析为范围语法（检查是否有闭包和范围）
    if let Ok(range_macro) = syn::parse::<RangeMacro>(input.clone()) {
        // 范围语法：max!(|i| d[i], [start..end])
        let closure = &range_macro.closure;
        let range = &range_macro.range;
        
        // 提取范围的开始和结束
        let start = range.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
        let end = range.end.as_ref().map(|e| quote! { #e });

        let expanded = if let Some(end_expr) = end {
            // 有结束范围的情况：start..end 或 start..=end
            let range_expr = match range.limits {
                syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
            };
            
            quote! {{
                let mut max_val = None;
                let mut max_index = 0;
                
                for __mau_idx in #range_expr {
                    let current_val = (#closure)(__mau_idx);
                    match max_val {
                        None => {
                            max_val = Some(current_val);
                            max_index = __mau_idx;
                        }
                        Some(max) => {
                            if current_val > max {
                                max_val = Some(current_val);
                                max_index = __mau_idx;
                            }
                        }
                    }
                }
                
                max_val.expect("Range cannot be empty")
            }}
        } else {
            // 无结束范围的情况：start..
            quote! {{
                let mut max_val = None;
                let mut max_index = 0;
                let mut __mau_idx = #start;
                
                loop {
                    let current_val = (#closure)(__mau_idx);
                    match max_val {
                        None => {
                            max_val = Some(current_val);
                            max_index = __mau_idx;
                        }
                        Some(max) => {
                            if current_val > max {
                                max_val = Some(current_val);
                                max_index = __mau_idx;
                            }
                        }
                    }
                    __mau_idx += 1;
                }
            }}
        };
        
        return expanded.into();
    }
    
    // 如果范围语法解析失败，尝试解析为多参数语法
    if let Ok(multi_args) = syn::parse::<MultiArgsMacro>(input) {
        // 多参数语法：max!(1, a, b, c, 3)
        if multi_args.args.is_empty() {
            return syn::Error::new(proc_macro2::Span::call_site(), "max! macro requires at least one argument").to_compile_error().into();
        }
        
        if multi_args.args.len() == 1 {
            // 只有一个参数，直接返回
            let arg = &multi_args.args[0];
            return quote! { #arg }.into();
        } else {
            // 多个参数，找到最大值
            let mut iter = multi_args.args.iter();
            let first = iter.next().unwrap();
            let rest = iter.collect::<Vec<_>>();
            
            return quote! {{
                let mut max_val = #first;
                #(
                    if #rest > max_val {
                        max_val = #rest;
                    }
                )*
                max_val
            }}.into();
        }
    }
    
    // 如果两种解析都失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), "Invalid syntax for max! macro. Use either max!(a, b, c) or max!(|i| expr, [start..end])").to_compile_error().into()
}

/// sum! 宏：在指定范围内求和
///
/// 语法：sum!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::sum;
///
/// let d = vec![1, 2, 3, 4, 5];
/// let sum_val = sum!(|i| d[i], [0..d.len()]);
/// assert_eq!(sum_val, 15);
/// ```
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    // 首先尝试解析为范围语法（检查是否有闭包和范围）
    if let Ok(range_macro) = syn::parse::<RangeMacro>(input.clone()) {
        // 范围语法：sum!(|i| d[i], [start..end])
        let closure = &range_macro.closure;
        let range = &range_macro.range;
        
        // 提取范围的开始和结束
        let start = range.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
        let end = range.end.as_ref().map(|e| quote! { #e });

        let expanded = if let Some(end_expr) = end {
            // 有结束范围的情况：start..end 或 start..=end
            let range_expr = match range.limits {
                syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
            };
            
            quote! {{
                let mut sum_val = None;
                
                for __mau_idx in #range_expr {
                    let current_val = (#closure)(__mau_idx);
                    sum_val = match sum_val {
                        None => Some(current_val),
                        Some(acc) => Some(acc + current_val),
                    };
                }
                
                sum_val.expect("Range cannot be empty")
            }}
        } else {
            // 无结束范围的情况：start..
            quote! {{
                let mut sum_val = None;
                let mut __mau_idx = #start;
                
                loop {
                    let current_val = (#closure)(__mau_idx);
                    sum_val = match sum_val {
                        None => Some(current_val),
                        Some(acc) => Some(acc + current_val),
                    };
                    __mau_idx += 1;
                }
            }}
        };
        
        return expanded.into();
    }
    
    // 如果范围语法解析失败，尝试解析为多参数语法
    if let Ok(multi_args) = syn::parse::<MultiArgsMacro>(input) {
        // 多参数语法：sum!(1, a, b, c, 3)
        if multi_args.args.is_empty() {
            return syn::Error::new(proc_macro2::Span::call_site(), "sum! macro requires at least one argument").to_compile_error().into();
        }
        
        if multi_args.args.len() == 1 {
            // 只有一个参数，直接返回
            let arg = &multi_args.args[0];
            return quote! { #arg }.into();
        } else {
            // 多个参数，求和
            let mut iter = multi_args.args.iter();
            let first = iter.next().unwrap();
            let rest = iter.collect::<Vec<_>>();
            
            return quote! {{
                let mut sum_val = #first;
                #(
                    sum_val = sum_val + #rest;
                )*
                sum_val
            }}.into();
        }
    }
    
    // 如果两种解析都失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), "Invalid syntax for sum! macro. Use either sum!(a, b, c) or sum!(|i| expr, [start..end])").to_compile_error().into()
}

/// and! 宏：在指定范围内进行逻辑与运算
///
/// 语法：and!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::and;
///
/// let d = vec![true, true, false, true];
/// let and_val = and!(|i| d[i], [0..d.len()]);
/// assert_eq!(and_val, false);
/// ```
#[proc_macro]
pub fn and(input: TokenStream) -> TokenStream {
    // 首先尝试解析为范围语法（检查是否有闭包和范围）
    if let Ok(range_macro) = syn::parse::<RangeMacro>(input.clone()) {
        // 范围语法：and!(|i| d[i], [start..end])
        let closure = &range_macro.closure;
        let range = &range_macro.range;
        
        // 提取范围的开始和结束
        let start = range.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
        let end = range.end.as_ref().map(|e| quote! { #e });

        let expanded = if let Some(end_expr) = end {
            // 有结束范围的情况：start..end 或 start..=end
            let range_expr = match range.limits {
                syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
            };
            
            quote! {{
                let mut and_val = true;
                
                for __mau_idx in #range_expr {
                    and_val = and_val && (#closure)(__mau_idx);
                    if !and_val {
                        break;
                    }
                }
                
                and_val
            }}
        } else {
            // 无结束范围的情况：start..
            quote! {{
                let mut and_val = true;
                let mut __mau_idx = #start;
                
                loop {
                    and_val = and_val && (#closure)(__mau_idx);
                    if !and_val {
                        break;
                    }
                    __mau_idx += 1;
                }
                
                and_val
            }}
        };
        
        return expanded.into();
    }
    
    // 如果范围语法解析失败，尝试解析为多参数语法
    if let Ok(multi_args) = syn::parse::<MultiArgsMacro>(input) {
        // 多参数语法：and!(true, a, b, c, false)
        if multi_args.args.is_empty() {
            return syn::Error::new(proc_macro2::Span::call_site(), "and! macro requires at least one argument").to_compile_error().into();
        }
        
        if multi_args.args.len() == 1 {
            // 只有一个参数，直接返回
            let arg = &multi_args.args[0];
            return quote! { #arg }.into();
        } else {
            // 多个参数，进行逻辑与运算
            let mut iter = multi_args.args.iter();
            let first = iter.next().unwrap();
            let rest = iter.collect::<Vec<_>>();
            
            return quote! {{
                let mut and_val = #first;
                #(
                    and_val = and_val && #rest;
                )*
                and_val
            }}.into();
        }
    }
    
    // 如果两种解析都失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), "Invalid syntax for and! macro. Use either and!(a, b, c) or and!(|i| expr, [start..end])").to_compile_error().into()
}

/// or! 宏：在指定范围内进行逻辑或运算
///
/// 语法：or!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::or;
///
/// let d = vec![false, false, true, false];
/// let or_val = or!(|i| d[i], [0..d.len()]);
/// assert_eq!(or_val, true);
/// ```
#[proc_macro]
pub fn or(input: TokenStream) -> TokenStream {
    // 首先尝试解析为范围语法（检查是否有闭包和范围）
    if let Ok(range_macro) = syn::parse::<RangeMacro>(input.clone()) {
        // 范围语法：or!(|i| d[i], [start..end])
        let closure = &range_macro.closure;
        let range = &range_macro.range;
        
        // 提取范围的开始和结束
        let start = range.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
        let end = range.end.as_ref().map(|e| quote! { #e });

        let expanded = if let Some(end_expr) = end {
            // 有结束范围的情况：start..end 或 start..=end
            let range_expr = match range.limits {
                syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
            };
            
            quote! {{
                let mut or_val = false;
                
                for __mau_idx in #range_expr {
                    or_val = or_val || (#closure)(__mau_idx);
                    if or_val {
                        break;
                    }
                }
                
                or_val
            }}
        } else {
            // 无结束范围的情况：start..
            quote! {{
                let mut or_val = false;
                let mut __mau_idx = #start;
                
                loop {
                    or_val = or_val || (#closure)(__mau_idx);
                    if or_val {
                        break;
                    }
                    __mau_idx += 1;
                }
                
                or_val
            }}
        };
        
        return expanded.into();
    }
    
    // 如果范围语法解析失败，尝试解析为多参数语法
    if let Ok(multi_args) = syn::parse::<MultiArgsMacro>(input) {
        // 多参数语法：or!(false, a, b, c, true)
        if multi_args.args.is_empty() {
            return syn::Error::new(proc_macro2::Span::call_site(), "or! macro requires at least one argument").to_compile_error().into();
        }
        
        if multi_args.args.len() == 1 {
            // 只有一个参数，直接返回
            let arg = &multi_args.args[0];
            return quote! { #arg }.into();
        } else {
            // 多个参数，进行逻辑或运算
            let mut iter = multi_args.args.iter();
            let first = iter.next().unwrap();
            let rest = iter.collect::<Vec<_>>();
            
            return quote! {{
                let mut or_val = #first;
                #(
                    or_val = or_val || #rest;
                )*
                or_val
            }}.into();
        }
    }
    
    // 如果两种解析都失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), "Invalid syntax for or! macro. Use either or!(a, b, c) or or!(|i| expr, [start..end])").to_compile_error().into()
}