//! # Mau - 宏工具库
//! 
//! 提供各种实用的过程宏，包括范围操作、记忆化缓存等。

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Token,
    parse::Parse, parse::ParseStream,
};

// 范围宏的解析结构
struct RangeMacro {
    closure: Expr,
    range: Expr,  // 改为Expr，支持任何可迭代的表达式
}

impl Parse for RangeMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析闭包 |i| d[i]
        let closure = input.parse::<Expr>()?;
        
        // 解析逗号
        input.parse::<Token![,]>()?;
        
        // 直接解析迭代器表达式，不再支持中括号
        let range = input.parse::<Expr>()?;
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

// 通用宏生成器
fn generate_macro_from_reduce(
    input: TokenStream,
    reduce_operation: &str,
    reduce_closure: proc_macro2::TokenStream,
) -> TokenStream {
    // 检查是否是短路操作
    let is_short_circuit = reduce_operation == "and" || reduce_operation == "or";
    
    // 首先尝试解析为范围语法
    if let Ok(range_macro) = syn::parse::<RangeMacro>(input.clone()) {
        // 范围语法：macro!(|i| d[i], [iterator]) 或 macro!(|i| d[i], [start..end])
        let closure = &range_macro.closure;
        let range = &range_macro.range;
        
        // 检查range是否是范围表达式（如 0..10 或 0..=10）
        let is_range_expr = match range {
            syn::Expr::Range(_range_expr) => true,
            _ => false,
        };
        
        let expanded = if is_range_expr {
            // 处理范围表达式：0..10 或 0..=10
            if let syn::Expr::Range(range_expr) = range {
                let start = range_expr.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
                let end = range_expr.end.as_ref().map(|e| quote! { #e });
                
                if let Some(end_expr) = end {
                    // 有结束范围的情况：[start..end] 或 [start..=end]
                    let range_expr = match range_expr.limits {
                        syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                        syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
                    };
                    
                    if is_short_circuit {
                        // 短路优化版本
                        if reduce_operation == "and" {
                            quote! {{
                                let mut result = true;
                                for __mau_idx in #range_expr {
                                    let current_val = (#closure)(__mau_idx);
                                    if !current_val {
                                        result = false;
                                        break;
                                    }
                                }
                                result
                            }}
                        } else { // or
                            quote! {{
                                let mut result = false;
                                for __mau_idx in #range_expr {
                                    let current_val = (#closure)(__mau_idx);
                                    if current_val {
                                        result = true;
                                        break;
                                    }
                                }
                                result
                            }}
                        }
                    } else {
                        // 普通归约版本
                        quote! {{
                            let mut result = None;
                            
                            for __mau_idx in #range_expr {
                                let current_val = (#closure)(__mau_idx);
                                result = match result {
                                    None => Some(current_val),
                                    Some(acc) => Some((#reduce_closure)(acc, current_val)),
                                };
                            }
                            
                            result.expect("Range cannot be empty")
                        }}
                    }
                } else {
                    // 无结束范围的情况：start..
                    if is_short_circuit {
                        // 短路优化版本
                        if reduce_operation == "and" {
                            quote! {{
                                let mut result = true;
                                let mut __mau_idx = #start;
                                loop {
                                    let current_val = (#closure)(__mau_idx);
                                    if !current_val {
                                        result = false;
                                        break;
                                    }
                                    __mau_idx += 1;
                                }
                                result
                            }}
                        } else { // or
                            quote! {{
                                let mut result = false;
                                let mut __mau_idx = #start;
                                loop {
                                    let current_val = (#closure)(__mau_idx);
                                    if current_val {
                                        result = true;
                                        break;
                                    }
                                    __mau_idx += 1;
                                }
                                result
                            }}
                        }
                    } else {
                        // 普通归约版本
                        quote! {{
                            let mut result = None;
                            let mut __mau_idx = #start;
                            
                            loop {
                                let current_val = (#closure)(__mau_idx);
                                result = match result {
                                    None => Some(current_val),
                                    Some(acc) => Some((#reduce_closure)(acc, current_val)),
                                };
                                __mau_idx += 1;
                            }
                        }}
                    }
                }
            } else {
                // 这不应该发生，因为我们已经检查了is_range_expr
                unreachable!()
            }
        } else {
            // 处理迭代器表达式：任何实现了IntoIterator的类型
            // 直接解引用，让闭包接收值而不是引用
            if is_short_circuit {
                // 短路优化版本
                if reduce_operation == "and" {
                    quote! {{
                        let mut result = true;
                        for __mau_item in #range {
                            let current_val = (#closure)(*__mau_item);
                            if !current_val {
                                result = false;
                                break;
                            }
                        }
                        result
                    }}
                } else { // or
                    quote! {{
                        let mut result = false;
                        for __mau_item in #range {
                            let current_val = (#closure)(*__mau_item);
                            if current_val {
                                result = true;
                                break;
                            }
                        }
                        result
                    }}
                }
            } else {
                // 普通归约版本
                quote! {{
                    let mut result = None;
                    
                    for __mau_item in #range {
                        let current_val = (#closure)(*__mau_item);
                        result = match result {
                            None => Some(current_val),
                            Some(acc) => Some((#reduce_closure)(acc, current_val)),
                        };
                    }
                    
                    result.expect("Iterator cannot be empty")
                }}
            }
        };
        
        return expanded.into();
    }
    
    // 如果范围语法解析失败，尝试解析为多参数语法
    if let Ok(multi_args) = syn::parse::<MultiArgsMacro>(input.clone()) {
        if multi_args.args.is_empty() {
            return syn::Error::new(proc_macro2::Span::call_site(), &format!("{}! macro requires at least one argument", reduce_operation)).to_compile_error().into();
        }
        
        if multi_args.args.len() == 1 {
            // 只有一个参数，检查是否为数组类型
            let arg = &multi_args.args[0];
            
            // 检查参数是否是数组/切片类型
            let is_array_like = match arg {
                syn::Expr::Path(_) => true,  // 变量名
                syn::Expr::Array(_) => true,  // 数组字面量
                syn::Expr::Call(call) => {
                    if let syn::Expr::Path(path) = &*call.func {
                        path.path.is_ident("vec") || path.path.is_ident("Vec")
                    } else {
                        false
                    }
                }
                _ => false,
            };
            
            if is_array_like {
                // 简写语法：macro!(array) -> macro!(|i| array[i], [0..array.len()])
                // 统一使用range语法处理
                let closure = quote! { |__mau_idx| #arg[__mau_idx] };
                let range_expr = quote! { 0..#arg.len() };
                
                if is_short_circuit {
                    // 短路优化版本
                    if reduce_operation == "and" {
                        return quote! {{
                            let mut result = true;
                            for __mau_idx in #range_expr {
                                let current_val = (#closure)(__mau_idx);
                                if !current_val {
                                    result = false;
                                    break;
                                }
                            }
                            result
                        }}.into();
                    } else { // or
                        return quote! {{
                            let mut result = false;
                            for __mau_idx in #range_expr {
                                let current_val = (#closure)(__mau_idx);
                                if current_val {
                                    result = true;
                                    break;
                                }
                            }
                            result
                        }}.into();
                    }
                } else {
                    // 普通归约版本
                    return quote! {{
                        let mut result = None;
                        
                        for __mau_idx in #range_expr {
                            let current_val = (#closure)(__mau_idx);
                            result = match result {
                                None => Some(current_val),
                                Some(acc) => Some((#reduce_closure)(acc, current_val)),
                            };
                        }
                        
                        result.expect("Array cannot be empty")
                    }}.into();
                }
            } else {
                // 单个非数组参数，直接返回
                return quote! { #arg }.into();
            }
        } else {
            // 多个参数：先生成数组，然后使用range语法统一处理
            let args = &multi_args.args;
            let array_len = args.len();
            
            // 生成数组：let __mau_array = [arg1, arg2, arg3, ...];
            let array_expr = quote! { [#(#args),*] };
            let closure = quote! { |__mau_idx| __mau_array[__mau_idx] };
            let range_expr = quote! { 0..#array_len };
            
            if is_short_circuit {
                // 短路优化版本
                if reduce_operation == "and" {
                    return quote! {{
                        let __mau_array = #array_expr;
                        let mut result = true;
                        for __mau_idx in #range_expr {
                            let current_val = (#closure)(__mau_idx);
                            if !current_val {
                                result = false;
                                break;
                            }
                        }
                        result
                    }}.into();
                } else { // or
                    return quote! {{
                        let __mau_array = #array_expr;
                        let mut result = false;
                        for __mau_idx in #range_expr {
                            let current_val = (#closure)(__mau_idx);
                            if current_val {
                                result = true;
                                break;
                            }
                        }
                        result
                    }}.into();
                }
            } else {
                // 普通归约版本
                return quote! {{
                    let __mau_array = #array_expr;
                    let mut result = None;
                    
                    for __mau_idx in #range_expr {
                        let current_val = (#closure)(__mau_idx);
                        result = match result {
                            None => Some(current_val),
                            Some(acc) => Some((#reduce_closure)(acc, current_val)),
                        };
                    }
                    
                    result.expect("Array cannot be empty")
                }}.into();
            }
        }
    }
    
    // 如果所有解析都失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), &format!("Invalid syntax for {}! macro. Use either {}!(a, b, c), {}!(array), or {}!(|i| expr, [start..end])", reduce_operation, reduce_operation, reduce_operation, reduce_operation)).to_compile_error().into()
}

/// min! 宏：在指定范围内找到最小值
/// 
/// 语法：min!(|i| d[i], [start..end])
/// 
/// 示例：
/// ```rust
/// use mau::min;
///
/// let d = vec![3, 1, 4, 1, 5, 9];
/// let min_val = min!(|i| d[i], 0..d.len());
/// assert_eq!(min_val, 1);
/// ```
#[proc_macro]
pub fn min(input: TokenStream) -> TokenStream {
    generate_macro_from_reduce(input, "min", quote! { |a, b| if a < b { a } else { b } })
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
/// let max_val = max!(|i| d[i], 0..d.len());
/// assert_eq!(max_val, 9);
/// ```
#[proc_macro]
pub fn max(input: TokenStream) -> TokenStream {
    generate_macro_from_reduce(input, "max", quote! { |a, b| if a > b { a } else { b } })
}

/// sum! 宏：在指定范围内求和
///
/// 语法：sum!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::sum;
///
/// let d = vec![3, 1, 4, 1, 5, 9];
/// let sum_val = sum!(|i| d[i], 0..d.len());
/// assert_eq!(sum_val, 23);
/// ```
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    generate_macro_from_reduce(input, "sum", quote! { |a, b| a + b })
}

/// and! 宏：在指定范围内进行逻辑与运算
///
/// 语法：and!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::and;
///
/// let bools = vec![true, true, false, true];
/// let and_val = and!(|i| bools[i], 0..bools.len());
/// assert_eq!(and_val, false);
/// ```
#[proc_macro]
pub fn and(input: TokenStream) -> TokenStream {
    generate_macro_from_reduce(input, "and", quote! { |a, b| a && b })
}

/// or! 宏：在指定范围内进行逻辑或运算
///
/// 语法：or!(|i| d[i], [start..end])
///
/// 示例：
/// ```rust
/// use mau::or;
///
/// let bools = vec![true, true, false, true];
/// let or_val = or!(|i| bools[i], 0..bools.len());
/// assert_eq!(or_val, true);
/// ```
#[proc_macro]
pub fn or(input: TokenStream) -> TokenStream {
    generate_macro_from_reduce(input, "or", quote! { |a, b| a || b })
}

// Reduce宏的解析结构
struct ReduceMacro {
    data_closure: Expr,
    range: Expr,  // 改为Expr，支持任何可迭代的表达式
    reduce_closure: Expr,
}

impl Parse for ReduceMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析数据闭包 |i| data[i]
        let data_closure = input.parse::<Expr>()?;
        
        // 解析逗号
        input.parse::<Token![,]>()?;
        
        // 直接解析迭代器表达式，不再支持中括号
        let range = input.parse::<Expr>()?;
        
        // 解析逗号
        input.parse::<Token![,]>()?;
        
        // 解析归约闭包 |a, b| if a > b { a } else { b }
        let reduce_closure = input.parse::<Expr>()?;
        
        Ok(ReduceMacro { 
            data_closure,
            range,
            reduce_closure,
        })
    }
}

/// reduce! 宏：在指定范围内进行归约操作
///
/// 语法：reduce!(|i| data[i], [start..end], |a, b| if a > b { a } else { b })
///
/// # 参数
/// - `data_closure`: 数据访问闭包，如 `|i| data[i]`
/// - `range`: 范围表达式，如 `[0..data.len()]` 或 `[2..6]`
/// - `reduce_closure`: 归约操作闭包，如 `|a, b| if a > b { a } else { b }`
///
/// # 示例
/// ```rust
/// use mau::reduce;
///
/// let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let max_val = reduce!(|i| data[i], 0..data.len(), |a, b| if a > b { a } else { b });
/// assert_eq!(max_val, 9);
///
/// // 也可以用于求和
/// let sum_val = reduce!(|i| data[i], 0..data.len(), |a, b| a + b);
/// assert_eq!(sum_val, 31);
///
/// // 部分范围
/// let partial_max = reduce!(|i| data[i], 2..6, |a, b| if a > b { a } else { b });
/// assert_eq!(partial_max, 9); // 只检查索引2到5: [4, 1, 5, 9]
/// ```
#[proc_macro]
pub fn reduce(input: TokenStream) -> TokenStream {
    // 解析reduce宏语法
    if let Ok(reduce_macro) = syn::parse::<ReduceMacro>(input) {
        let data_closure = &reduce_macro.data_closure;
        let range = &reduce_macro.range;
        let reduce_closure = &reduce_macro.reduce_closure;
        
        // 检查range是否是范围表达式（如 0..10 或 0..=10）
        let is_range_expr = match range {
            syn::Expr::Range(_) => true,
            _ => false,
        };

        let expanded = if is_range_expr {
            // 处理范围表达式：0..10 或 0..=10
            if let syn::Expr::Range(range_expr) = range {
                let start = range_expr.start.as_ref().map(|s| quote! { #s }).unwrap_or(quote! { 0 });
                let end = range_expr.end.as_ref().map(|e| quote! { #e });

                if let Some(end_expr) = end {
                    // 有结束范围的情况：[start..end] 或 [start..=end]
                    let range_expr = match range_expr.limits {
                        syn::RangeLimits::HalfOpen(_) => quote! { #start..#end_expr },
                        syn::RangeLimits::Closed(_) => quote! { #start..=#end_expr },
                    };

                    quote! {{
                        let mut result = None;

                        for __mau_idx in #range_expr {
                            let current_val = (#data_closure)(__mau_idx);
                            result = match result {
                                None => Some(current_val),
                                Some(acc) => Some((#reduce_closure)(acc, current_val)),
                            };
                        }

                        result.expect("Range cannot be empty")
                    }}
                } else {
                    // 无结束范围的情况：start..
                    quote! {{
                        let mut result = None;
                        let mut __mau_idx = #start;

                        loop {
                            let current_val = (#data_closure)(__mau_idx);
                            result = match result {
                                None => Some(current_val),
                                Some(acc) => Some((#reduce_closure)(acc, current_val)),
                            };
                            __mau_idx += 1;
                        }
                    }}
                }
            } else {
                // 这不应该发生，因为我们已经检查了is_range_expr
                unreachable!()
            }
        } else {
            // 处理迭代器表达式：任何实现了IntoIterator的类型
            quote! {{
                let mut result = None;

                for __mau_item in #range {
                    let current_val = (#data_closure)(__mau_item);
                    result = match result {
                        None => Some(current_val),
                        Some(acc) => Some((#reduce_closure)(acc, current_val)),
                    };
                }

                result.expect("Iterator cannot be empty")
            }}
        };
        
        return expanded.into();
    }
    
    // 如果解析失败，返回错误
    syn::Error::new(proc_macro2::Span::call_site(), "Invalid syntax for reduce! macro. Use reduce!(|i| data[i], [start..end], |a, b| operation)").to_compile_error().into()
}

// 从备份文件中提取memo宏的实现
use syn::{
    parse_macro_input, FnArg, Ident, ItemFn, Pat, PatIdent, ReturnType, Type,
    punctuated::Punctuated, spanned::Spanned,
};
use std::collections::HashSet;

// KeyArgs 结构用于解析 memo 宏的属性参数
struct KeyArgs {
    args: Punctuated<Ident, syn::Token![,]>,
}

impl syn::parse::Parse for KeyArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = Punctuated::parse_terminated(input)?;
        Ok(KeyArgs { args })
    }
}

/// memo 宏：为函数添加记忆化缓存
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
                        return Err(syn::Error::new(
                            pat_type.span(),
                            "only simple identifiers are supported",
                        ))
                    }
                };
                let ty = &*pat_type.ty;
                Ok((ident, ty))
            }
            _ => Err(syn::Error::new(arg.span(), "self parameters are not supported")),
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
                        return syn::Error::new(ty_ref.span(), format!("memo supports only immutable references in parameters, but {} is mutable", pat_ident.ident))
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

    let create_cache = quote! {
        static #cache_name: ::std::sync::LazyLock<::std::sync::Mutex<::std::collections::HashMap<#key_type, #return_type>>> = ::std::sync::LazyLock::new(|| {
            ::std::sync::Mutex::new(::std::collections::HashMap::new())
        });
    };

    let no_cache_fn = quote! {
        #fn_vis fn #no_cache_name(#fn_inputs) #fn_output #fn_block
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