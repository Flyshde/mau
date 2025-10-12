//! # Mau - 宏工具库
//! 
//! 提供各种实用的过程宏，包括范围操作、记忆化缓存等。

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Token, Type, Ident, Pat, PatIdent, FnArg,
    parse::Parse, parse::ParseStream,
    visit_mut::VisitMut,
};
use std::collections::HashSet;

// 辅助函数：将下划线命名转为大驼峰命名
fn to_upper_camel_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

// ref 模式专用：生成自定义键类型的代码
// RefKey<T>: 统一处理所有 &T 类型
//   - addr: 地址，用于快速比较（相同地址 = 同一个引用）
//   - content: T 的克隆，用于慢速比较（不同地址但内容相同）
// 为每个函数生成唯一的 RefKey 类型，避免冲突
fn generate_ref_key_struct(fn_name: &Ident) -> proc_macro2::TokenStream {
    let ref_key_name = Ident::new(
        &format!("RefKey{}", to_upper_camel_case(&fn_name.to_string())),
        fn_name.span()
    );
    quote! {
        #[derive(Clone)]
        struct #ref_key_name<T> {
            addr: usize,
            content: T,
        }

        impl<T: PartialEq> PartialEq for #ref_key_name<T> {
            fn eq(&self, other: &Self) -> bool {
                // 先比地址（快速路径）
                if self.addr == other.addr {
                    return true;  // 相同地址，直接返回 true！
                }
                // 地址不同，比较内容（慢速路径）
                self.content == other.content
            }
        }

        impl<T: Eq> Eq for #ref_key_name<T> {}

        impl<T: std::hash::Hash> std::hash::Hash for #ref_key_name<T> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                // 只基于内容 hash，内容相同进入同一个 HashMap 桶
                self.content.hash(state);
            }
        }
    }
}

// 辅助函数：生成空迭代器时返回边界值的代码
fn generate_empty_handler(operation: &str) -> proc_macro2::TokenStream {
    match operation {
        "min" => quote! {
            match result {
                Some(v) => v,
                None => {
                    // 只有空迭代器才会执行到这里
                    // 运行时类型检查，返回对应类型的 MAX 值
                    use ::std::any::TypeId;
                    
                    fn get_max_value<T: 'static + Copy>() -> T {
                        let tid = TypeId::of::<T>();
                        
                        macro_rules! try_return {
                            ($t:ty) => {
                                if tid == TypeId::of::<$t>() {
                                    let val = <$t>::MAX;
                                    return unsafe { ::std::mem::transmute_copy(&val) };
                                }
                            };
                        }
                        
                        try_return!(i8);
                        try_return!(i16);
                        try_return!(i32);
                        try_return!(i64);
                        try_return!(i128);
                        try_return!(isize);
                        try_return!(u8);
                        try_return!(u16);
                        try_return!(u32);
                        try_return!(u64);
                        try_return!(u128);
                        try_return!(usize);
                        try_return!(f32);
                        try_return!(f64);
                        
                        if tid == TypeId::of::<char>() {
                            let val = '\u{10FFFF}';
                            return unsafe { ::std::mem::transmute_copy(&val) };
                        }
                        
                        panic!("min! macro: type does not have a MAX value, empty iterator not supported")
                    }
                    
                    get_max_value()
                }
            }
        },
        "max" => quote! {
            match result {
                Some(v) => v,
                None => {
                    // 只有空迭代器才会执行到这里
                    // 运行时类型检查，返回对应类型的 MIN 值
                    use ::std::any::TypeId;
                    
                    fn get_min_value<T: 'static + Copy>() -> T {
                        let tid = TypeId::of::<T>();
                        
                        macro_rules! try_return {
                            ($t:ty) => {
                                if tid == TypeId::of::<$t>() {
                                    let val = <$t>::MIN;
                                    return unsafe { ::std::mem::transmute_copy(&val) };
                                }
                            };
                        }
                        
                        try_return!(i8);
                        try_return!(i16);
                        try_return!(i32);
                        try_return!(i64);
                        try_return!(i128);
                        try_return!(isize);
                        try_return!(u8);
                        try_return!(u16);
                        try_return!(u32);
                        try_return!(u64);
                        try_return!(u128);
                        try_return!(usize);
                        try_return!(f32);
                        try_return!(f64);
                        
                        if tid == TypeId::of::<char>() {
                            let val = '\0';
                            return unsafe { ::std::mem::transmute_copy(&val) };
                        }
                        
                        panic!("max! macro: type does not have a MIN value, empty iterator not supported")
                    }
                    
                    get_min_value()
                }
            }
        },
        _ => quote! {
            result.expect("Iterator cannot be empty")
        }
    }
}

// 辅助函数：生成 ptr 模式的键（地址 + 长度）
fn generate_ptr_mode_key(
    arg: &Ident,
    ty: &Type,
    key_types: &mut Vec<proc_macro2::TokenStream>,
    key_exprs: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Type::Reference(ty_ref) = ty {
        if let Type::Slice(_slice_ty) = &*ty_ref.elem {
            // 对于切片 &[T]，使用 (地址, 长度) 作为键
            key_types.push(quote! { (usize, usize) });
            key_exprs.push(quote! { (#arg.as_ptr() as usize, #arg.len()) });
        } else if let Type::Array(_array_ty) = &*ty_ref.elem {
            // 对于数组 &[T; N]，使用 (地址, 长度) 作为键
            key_types.push(quote! { (usize, usize) });
            key_exprs.push(quote! { (#arg.as_ptr() as usize, #arg.len()) });
        } else {
            // 对于其他引用类型，只使用地址
            key_types.push(quote! { usize });
            key_exprs.push(quote! { #arg.as_ptr() as usize });
        }
    } else {
        // 对于非引用类型，直接使用
        key_types.push(quote! { #ty });
        key_exprs.push(quote! { #arg });
    }
}

// 辅助函数：生成 ref 模式的键（使用 RefKey 包装类型）
// RefKey<T> 实现了自定义的 Eq 和 Hash：
//   - Hash: 只基于内容（内容相同进入同一个 HashMap 桶）
//   - Eq: 先比地址（快），地址相同直接返回 true；地址不同再比内容（慢）
// 
// 统一处理逻辑：
//   - 对于 &[T] 或 &[T; N]：使用 RefKey<Vec<T>>（因为 [T] 不是 Sized）
//   - 对于 &T（其他）：使用 RefKey<T>
//   - 对于 T（非引用）：直接使用 T
fn generate_normal_mode_key(
    fn_name: &Ident,
    arg: &Ident,
    ty: &Type,
    key_types: &mut Vec<proc_macro2::TokenStream>,
    key_exprs: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Type::Reference(ty_ref) = ty {
        let ref_key_name = Ident::new(
            &format!("RefKey{}", to_upper_camel_case(&fn_name.to_string())),
            fn_name.span()
        );
        let inner_ty = &ty_ref.elem;
        
        // 检查内部类型并生成对应的 RefKey
        match &**inner_ty {
            Type::Slice(slice_ty) => {
                // 对于 &[T]，检查元素类型
                let elem_ty = &*slice_ty.elem;  // 解引用 Box
                
                // 特殊处理 f64
                if let Type::Path(type_path) = elem_ty {
                    if let Some(ident) = type_path.path.get_ident() {
                        if ident == "f64" {
                            key_types.push(quote! { #ref_key_name<Vec<u64>> });
                            key_exprs.push(quote! {
                                #ref_key_name {
                                    addr: #arg.as_ptr() as usize,
                                    content: #arg.iter().map(|x| x.to_bits()).collect(),
                                }
                            });
                            return;
                        }
                    }
                }
                
                // 特殊处理 &[[f64; N]] 类型
                if let Type::Array(inner_array) = elem_ty {
                    if let Type::Path(type_path) = &*inner_array.elem {
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "f64" {
                                key_types.push(quote! { #ref_key_name<Vec<Vec<u64>>> });
                                key_exprs.push(quote! {
                                    #ref_key_name {
                                        addr: #arg.as_ptr() as usize,
                                        content: #arg.iter().map(|row| row.iter().map(|x| x.to_bits()).collect()).collect(),
                                    }
                                });
                                return;
                            }
                        }
                    }
                }
                
                // 其他类型
                key_types.push(quote! { #ref_key_name<Vec<#elem_ty>> });
                key_exprs.push(quote! {
                    #ref_key_name {
                        addr: #arg.as_ptr() as usize,
                        content: #arg.to_vec(),
                    }
                });
            }
            Type::Array(array_ty) => {
                // 对于 &[T; N]，检查元素类型
                let elem_ty = &*array_ty.elem;  // 解引用 Box
                
                // 特殊处理 f64
                if let Type::Path(type_path) = elem_ty {
                    if let Some(ident) = type_path.path.get_ident() {
                        if ident == "f64" {
                            key_types.push(quote! { #ref_key_name<Vec<u64>> });
                            key_exprs.push(quote! {
                                #ref_key_name {
                                    addr: #arg.as_ptr() as usize,
                                    content: #arg.iter().map(|x| x.to_bits()).collect(),
                                }
                            });
                            return;
                        }
                    }
                }
                
                // 特殊处理 &[[f64; M]; N] 类型（嵌套数组）
                if let Type::Array(inner_array) = elem_ty {
                    if let Type::Path(type_path) = &*inner_array.elem {
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "f64" {
                                key_types.push(quote! { #ref_key_name<Vec<Vec<u64>>> });
                                key_exprs.push(quote! {
                                    #ref_key_name {
                                        addr: #arg.as_ptr() as usize,
                                        content: #arg.iter().map(|row| row.iter().map(|x| x.to_bits()).collect()).collect(),
                                    }
                                });
                                return;
                            }
                        }
                    }
                }
                
                // 其他类型
                key_types.push(quote! { #ref_key_name<Vec<#elem_ty>> });
                key_exprs.push(quote! {
                    #ref_key_name {
                        addr: #arg.as_ptr() as usize,
                        content: #arg.to_vec(),
                    }
                });
            }
            _ => {
                // 对于其他类型（如 &i32, &String），使用 RefKey<T>
                
                // 特殊处理 f64
                if let Type::Path(type_path) = &**inner_ty {
                    if let Some(ident) = type_path.path.get_ident() {
                        if ident == "f64" {
                            key_types.push(quote! { #ref_key_name<u64> });
                            key_exprs.push(quote! {
                                #ref_key_name {
                                    addr: #arg as *const _ as usize,
                                    content: #arg.to_bits(),
                                }
                            });
                            return;
                        }
                    }
                }
                
                // 其他类型
                key_types.push(quote! { #ref_key_name<#inner_ty> });
                key_exprs.push(quote! {
                    #ref_key_name {
                        addr: #arg as *const _ as usize,
                        content: (*#arg).clone(),
                    }
                });
            }
        }
    } else {
        // 对于非引用类型，直接使用
        key_types.push(quote! { #ty });
        key_exprs.push(quote! { #arg });
    }
}

// 辅助函数：生成 heavy 模式的键
fn generate_heavy_mode_key(
    arg: &Ident,
    ty: &Type,
    key_types: &mut Vec<proc_macro2::TokenStream>,
    key_exprs: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Type::Reference(ty_ref) = ty {
        if let Type::Slice(slice_ty) = &*ty_ref.elem {
            // 对于 &[T] 类型，完全还原为 [T]
            let elem_ty = &*slice_ty.elem;
            if let Type::Path(type_path) = elem_ty {
                if let Some(ident) = type_path.path.get_ident() {
                    if ident == "f64" {
                        // 对于 &[f64] 类型，使用 Vec<u64> 作为键
                        key_types.push(quote! { Vec<u64> });
                        key_exprs.push(quote! { #arg.iter().map(|x| x.to_bits()).collect() });
                    } else {
                        // 对于其他切片类型，使用 Vec<T> 作为键
                        key_types.push(quote! { Vec<#elem_ty> });
                        key_exprs.push(quote! { #arg.to_vec() });
                    }
                } else {
                    key_types.push(quote! { Vec<#elem_ty> });
                    key_exprs.push(quote! { #arg.to_vec() });
                }
            } else if let Type::Array(array_ty) = elem_ty {
                // 对于 &[[T; N]] 类型，完全还原为 [[T; N]]
                let inner_elem_ty = &*array_ty.elem;
                if let Type::Path(type_path) = inner_elem_ty {
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
                key_types.push(quote! { Vec<#elem_ty> });
                key_exprs.push(quote! { #arg.to_vec() });
            }
        } else if let Type::Array(array_ty) = &*ty_ref.elem {
            // 对于 &[T; N] 类型，完全还原为 [T; N]
            let elem_ty = &*array_ty.elem;
            if let Type::Path(type_path) = elem_ty {
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
            // 对于其他引用类型，完全还原
            let elem_ty = &ty_ref.elem;
            if let Type::Path(type_path) = &**elem_ty {
                if let Some(ident) = type_path.path.get_ident() {
                    if ident == "f64" {
                        // 对于 f64 类型，使用 u64 作为键（通过位模式转换）
                        key_types.push(quote! { u64 });
                        key_exprs.push(quote! { #arg.to_bits() });
                    } else {
                        // 对于其他类型，直接克隆
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
        // 对于非引用类型，直接使用
        key_types.push(quote! { #ty });
        key_exprs.push(quote! { #arg });
    }
}

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
    
    // 生成空迭代器处理代码
    let empty_handler = generate_empty_handler(reduce_operation);
    
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
                            
                            #empty_handler
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
                    
                    #empty_handler
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
                    
                        #empty_handler
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
    parse_macro_input, ItemFn, ReturnType, Item,
    punctuated::Punctuated, spanned::Spanned,
};

// KeyArgs 结构用于解析 memo 宏的属性参数
struct KeyArgs {
    args: Punctuated<Ident, syn::Token![,]>,
    named_args: std::collections::HashMap<String, String>,
}

impl syn::parse::Parse for KeyArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Punctuated::new();
        let mut named_args = std::collections::HashMap::new();
        
        while !input.is_empty() {
            // 尝试解析命名参数 key=value
            if input.peek(Ident) && input.peek2(syn::Token![=]) {
                let key: Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                
                // 解析值（可能是关键字 ref）
                let value_str = if input.peek(syn::Token![ref]) {
                    input.parse::<syn::Token![ref]>()?;
                    "ref".to_string()
                } else {
                    let value: Ident = input.parse()?;
                    value.to_string()
                };
                
                named_args.insert(key.to_string(), value_str);
            } else if input.peek(Ident) {
                // 位置参数（向后兼容）
                let arg: Ident = input.parse()?;
                args.push(arg);
            } else {
                break;
            }
            
            // 解析逗号
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            } else {
                break;
            }
        }
        
        Ok(KeyArgs { args, named_args })
    }
}

// 解析线程模式和键模式的辅助函数
fn parse_memo_modes(key_args: &KeyArgs) -> (String, String) {
    // 首先检查命名参数
    if !key_args.named_args.is_empty() {
        // 验证只有 thread 和 key 两个参数
        for (k, _) in &key_args.named_args {
            if k != "thread" && k != "key" {
                panic!("无效的参数名 '{}'. 只支持 'thread' 和 'key'", k);
            }
        }
        
        let thread_mode = key_args
            .named_args
            .get("thread")
            .map(|s| {
                // 验证 thread 模式
                match s.as_str() {
                    "single" | "multi" => s.clone(),
                    _ => panic!("无效的 thread 模式 '{}'. 只支持 'single' 或 'multi'", s),
                }
            })
            .unwrap_or_else(|| "single".to_string());
        
        let key_mode = key_args
            .named_args
            .get("key")
            .map(|s| {
                // 验证 key 模式
                match s.as_str() {
                    "ptr" | "ref" | "val" => s.clone(),
                    _ => panic!("无效的 key 模式 '{}'. 只支持 'ptr', 'ref' 或 'val'", s),
                }
            })
            .unwrap_or_else(|| "ref".to_string());
        
        return (thread_mode, key_mode);
    }
    
    // 向后兼容：位置参数（映射旧名称到新名称）
    if key_args.args.is_empty() {
        return ("single".to_string(), "ref".to_string());
    }
    
    // 检查第一个参数
        if let Some(first_arg) = key_args.args.first() {
            let arg_str = first_arg.to_string();
        
        // 映射旧的线程模式名称
        let thread_mode = match arg_str.as_str() {
            "local" => "single".to_string(),  // local -> single
            "single" => {
                // 旧的 single -> 现在不存在，报错或使用默认
                eprintln!("警告：旧的 'single' 模式已被移除，请使用 'single'(原local) 或 'multi'");
                "single".to_string()
            }
            "multi" => "multi".to_string(),
            // 检查是否是键模式
            "light" => {
                // 这是键模式，使用默认线程
                return ("single".to_string(), "ptr".to_string());
            }
            "normal" => {
                return ("single".to_string(), "ref".to_string());
            }
            "heavy" => {
                return ("single".to_string(), "val".to_string());
            }
            "ptr" | "ref" | "val" => {
                // 新的键模式名称
                let key_mode = match arg_str.as_str() {
                    "ptr" => "ptr".to_string(),
                    "ref" => "ref".to_string(),
                    "val" => "val".to_string(),
                    _ => "ref".to_string(),
                };
                return ("single".to_string(), key_mode);
            }
            _ => "single".to_string(),
        };
        
        // 检查第二个参数
        let key_mode = if key_args.args.len() > 1 {
                    let second_arg = &key_args.args[1];
                    let second_str = second_arg.to_string();
            match second_str.as_str() {
                // 旧名称映射
                "light" => "ptr".to_string(),
                "normal" => "ref".to_string(),
                "heavy" => "val".to_string(),
                // 新名称
                "ptr" | "ref" | "val" => second_str,
                _ => "ref".to_string(),
                    }
                } else {
            "ref".to_string()
        };
        
        (thread_mode, key_mode)
            } else {
        ("single".to_string(), "ref".to_string())
    }
}

/// memo 宏：为函数添加记忆化缓存
#[proc_macro_attribute]
pub fn memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let key_args = parse_macro_input!(attr as KeyArgs);
    
    // 解析线程模式和索引模式
    let (thread_mode, index_mode) = parse_memo_modes(&key_args);

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

    // 根据索引模式处理引用参数
    let mut key_args = Vec::new();
    let mut key_types = Vec::new();
    let mut key_exprs = Vec::new();
    
    for (arg, ty) in args.iter().zip(param_types.into_iter()) {
        key_args.push(arg.clone());
        
        if immutable_references.contains(&arg.to_string()) {
            // 根据键模式处理引用参数
            match index_mode.as_str() {
                "ptr" => {
                    // ptr 模式：使用 (地址, 长度) 作为键（原 light）
                    generate_ptr_mode_key(&arg, &ty, &mut key_types, &mut key_exprs);
                },
                "ref" => {
                    // ref 模式：解开一层引用（原 normal，默认）
                    generate_normal_mode_key(fn_name, &arg, &ty, &mut key_types, &mut key_exprs);
                },
                "val" => {
                    // val 模式：完全还原（原 heavy）
                    generate_heavy_mode_key(&arg, &ty, &mut key_types, &mut key_exprs);
                },
                _ => {
                    // 默认使用 ref 模式
                    generate_normal_mode_key(fn_name, &arg, &ty, &mut key_types, &mut key_exprs);
                }
            }
        } else {
            // 对于非引用参数，克隆参数以避免移动
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

    let (create_cache, cache_impl) = if thread_mode == "multi" {
        // Multi 模式：使用 Mutex<HashMap>，支持多线程
        let create_cache = quote! {
            static #cache_name: ::std::sync::LazyLock<::std::sync::Mutex<::std::collections::HashMap<#key_type, #return_type>>> = ::std::sync::LazyLock::new(|| {
                ::std::sync::Mutex::new(::std::collections::HashMap::new())
            });
        };
        
        let cache_impl = quote! {
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
        };
        
        (create_cache, cache_impl)
    } else {
        // Single 模式（默认）：使用 thread_local!，真正的单线程，无锁
        let create_cache = quote! {
            ::std::thread_local! {
                static #cache_name: ::std::cell::RefCell<::std::collections::HashMap<#key_type, #return_type>> = ::std::cell::RefCell::new(::std::collections::HashMap::new());
            }
        };
        
        let cache_impl = quote! {
            let cache_key = #key_tuple;
            // 检查缓存
            #cache_name.with(|cache| {
                // 先检查缓存
                if let Some(result) = cache.borrow().get(&cache_key) {
                    return result.clone();
                }
                // 计算并缓存结果
                let result = #no_cache_name(#(#call_args),*);
                cache.borrow_mut().insert(cache_key, result.clone());
                result
            })
        };
        
        (create_cache, cache_impl)
    };

    let no_cache_fn = quote! {
        #fn_vis fn #no_cache_name(#fn_inputs) #fn_output #fn_block
    };

    // 检查是否需要生成 RefKey 结构体
    // 只要 key_type 中包含 "RefKey"，就需要生成结构体定义
    let key_type_str = key_type.to_string();
    let ref_key_struct = if key_type_str.contains("RefKey") {
        generate_ref_key_struct(fn_name)
    } else {
        quote! {}
    };

    // 重新定义原始函数，添加缓存功能
    let expanded = quote! {
        #ref_key_struct
        #create_cache
        #no_cache_fn
        
        // 重新定义原始函数名，添加缓存逻辑
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            #cache_impl
        }
    };
    
    expanded.into()
}

// 共享的辅助函数：生成缓存键（供 memo 和 memo_block 复用）
fn generate_cache_keys(
    fn_name: &Ident,
    args: &[(Ident, Type)],
    key_mode: &str,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut key_types = Vec::new();
    let mut key_exprs = Vec::new();
    
    // 提取参数中的不可变引用
    let mut immutable_references = HashSet::<String>::new();
    for (arg, ty) in args {
        if let Type::Reference(ty_ref) = ty {
            if ty_ref.mutability.is_none() {
                immutable_references.insert(arg.to_string());
            }
        }
    }
    
    for (arg, ty) in args {
        if immutable_references.contains(&arg.to_string()) {
            // 根据键模式处理引用参数
            match key_mode {
                "ptr" => {
                    // ptr 模式：使用 (地址, 长度) 作为键（原 light）
                    generate_ptr_mode_key(&arg, &ty, &mut key_types, &mut key_exprs);
                }
                "ref" => {
                    // ref 模式：解开一层引用（原 normal，默认）
                    generate_normal_mode_key(fn_name, &arg, &ty, &mut key_types, &mut key_exprs);
                }
                "val" => {
                    // val 模式：完全还原（原 heavy）
                    generate_heavy_mode_key(&arg, &ty, &mut key_types, &mut key_exprs);
                }
                _ => {
                    // 默认使用 ref 模式
                    generate_normal_mode_key(fn_name, &arg, &ty, &mut key_types, &mut key_exprs);
                }
            }
        } else {
            // 对于非引用参数，克隆参数
            key_types.push(quote! { #ty });
            key_exprs.push(quote! { #arg.clone() });
        }
    }
    
    (key_types, key_exprs)
}

// Visitor：将函数体中的函数调用替换为内部函数调用
struct ReplaceCallVisitor {
    name_map: std::collections::HashMap<String, Ident>,
}

impl VisitMut for ReplaceCallVisitor {
    fn visit_expr_call_mut(&mut self, node: &mut syn::ExprCall) {
        syn::visit_mut::visit_expr_call_mut(self, node);
        
        if let Expr::Path(expr_path) = &*node.func {
            if let Some(ident) = expr_path.path.get_ident() {
                let fn_name = ident.to_string();
                if let Some(inner_name) = self.name_map.get(&fn_name) {
                    let mut new_path = expr_path.clone();
                    new_path.path.segments.last_mut().unwrap().ident = inner_name.clone();
                    node.func = Box::new(Expr::Path(new_path));
                }
            }
        }
    }
}

// MemoBlock 结构
struct MemoBlock {
    items: Vec<ItemFn>,
}

impl Parse for MemoBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        
        while !input.is_empty() {
            if let Ok(item) = input.parse::<Item>() {
                match item {
                    Item::Fn(func) => {
                        items.push(func);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            item.span(),
                            "memo_block! only accepts function definitions",
                        ));
                    }
                }
            } else {
                break;
            }
        }
        
        Ok(MemoBlock { items })
    }
}

// 辅助函数：从函数的属性中解析 memo 参数
fn parse_fn_attributes(attrs: &[syn::Attribute]) -> KeyArgs {
    // 查找第一个有效属性
    for attr in attrs {
        // 尝试解析为 KeyArgs（#[cfg(thread=multi, key=ptr)] 形式）
        if let Ok(key_args) = attr.parse_args::<KeyArgs>() {
            return key_args;
        }
    }
    
    // 没有属性或解析失败，返回空
    KeyArgs {
        args: Punctuated::new(),
        named_args: std::collections::HashMap::new(),
    }
}

/// memo_block! 宏：为多个函数添加带自动清理的记忆化缓存
#[proc_macro]
pub fn memo_block(input: TokenStream) -> TokenStream {
    let memo_block = parse_macro_input!(input as MemoBlock);
    
    // 默认使用 single 线程模式和 ref 键模式
    let default_thread_mode = "single";
    let default_key_mode = "ref";
    
    // 构建名称映射表
    let mut name_map = std::collections::HashMap::new();
    for func in &memo_block.items {
        let fn_name = func.sig.ident.to_string();
        let inner_name = Ident::new(&format!("{}_inner", fn_name), func.sig.ident.span());
        name_map.insert(fn_name, inner_name);
    }
    
    let mut output = Vec::new();
    
    for mut func in memo_block.items {
        // 从函数的属性中解析参数
        let fn_key_args = parse_fn_attributes(&func.attrs);
        let (thread_mode, key_mode) = parse_memo_modes(&fn_key_args);
        
        // 如果解析结果为空，使用默认值
        let thread_mode = if thread_mode.is_empty() || 
                             (fn_key_args.args.is_empty() && fn_key_args.named_args.is_empty()) {
            default_thread_mode
        } else {
            &thread_mode
        };
        
        let key_mode = if key_mode.is_empty() || 
                          (fn_key_args.args.is_empty() && fn_key_args.named_args.is_empty()) {
            default_key_mode
        } else {
            &key_mode
        };
        
        // 清除属性（避免生成到最终代码中）
        func.attrs.clear();
        let fn_name = &func.sig.ident;
        let fn_vis = &func.vis;
        let fn_inputs = &func.sig.inputs;
        let fn_output = &func.sig.output;
        
        // 替换函数体中的调用
        let mut visitor = ReplaceCallVisitor {
            name_map: name_map.clone(),
        };
        visitor.visit_block_mut(&mut func.block);
        let fn_block = &func.block;
        
        let inner_name = Ident::new(&format!("{}_inner", fn_name), fn_name.span());
        let clear_name = Ident::new(&format!("clear_{}", fn_name), fn_name.span());
        let cache_name = Ident::new(
            &format!("{}_CACHE", fn_name.to_string().to_uppercase()),
            fn_name.span(),
        );
        
        // 提取参数和类型
        let args_and_types: Vec<(Ident, Type)> = func
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(pat_type) => {
                    let ident = match &*pat_type.pat {
                        Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
                        _ => return None,
                    };
                    let ty = (*pat_type.ty).clone();
                    Some((ident, ty))
                }
                _ => None,
            })
            .collect();
        
        // 如果没有参数，直接返回原函数
        if args_and_types.is_empty() {
            output.push(quote! {
                #fn_vis fn #fn_name(#fn_inputs) #fn_output #fn_block
            });
            continue;
        }
        
        // 使用共享的参数键生成函数（复用 ptr/ref/val 逻辑）
        let (key_types, key_exprs) = generate_cache_keys(fn_name, &args_and_types, key_mode);
        
        let key_type = if key_types.len() == 1 {
            quote! { #(#key_types)* }
        } else {
            quote! { (#(#key_types),*) }
        };
        
        let key_tuple = if key_exprs.len() == 1 {
            quote! { #(#key_exprs)* }
        } else {
            quote! { (#(#key_exprs),*) }
        };
        
        let call_args: Vec<_> = args_and_types.iter().map(|(arg, _)| quote! { #arg }).collect();
        
        let return_type = match fn_output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => quote! { #ty },
        };
        
        // 根据线程模式生成缓存和清理函数
        let (create_cache, clear_impl) = if thread_mode == "multi" {
            (
                quote! {
                    static #cache_name: ::std::sync::LazyLock<::std::sync::Mutex<::std::collections::HashMap<#key_type, #return_type>>> = ::std::sync::LazyLock::new(|| {
                        ::std::sync::Mutex::new(::std::collections::HashMap::new())
                    });
                },
                quote! {
                    let mut cache = #cache_name.lock().unwrap();
                    cache.clear();
                },
            )
        } else {
            // single 模式（默认）：使用 thread_local
            (
                quote! {
                    ::std::thread_local! {
                        static #cache_name: ::std::cell::RefCell<::std::collections::HashMap<#key_type, #return_type>> = ::std::cell::RefCell::new(::std::collections::HashMap::new());
                    }
                },
                quote! {
                    #cache_name.with(|cache| cache.borrow_mut().clear());
                },
            )
        };
        
        // 生成缓存检查和存储的逻辑
        let (cache_check, cache_insert) = if thread_mode == "multi" {
            (
                quote! {
                    {
                        let cache = #cache_name.lock().unwrap();
                if let Some(result) = cache.get(&cache_key) {
                    return result.clone();
                }
            }
                },
                quote! {
                    {
                        let mut cache = #cache_name.lock().unwrap();
            cache.insert(cache_key, result.clone());
                    }
                },
            )
        } else {
            // single 模式（默认）：thread_local
            (
                quote! {
                    let found = #cache_name.with(|cache| {
                        cache.borrow().get(&cache_key).cloned()
                    });
                    if let Some(result) = found {
                        return result;
                    }
                },
                quote! {
                    #cache_name.with(|cache| {
                        cache.borrow_mut().insert(cache_key, result.clone());
                    });
                },
            )
        };
        
        output.push(quote! {
            #create_cache
            
            // 清理函数
            #fn_vis fn #clear_name() {
                #clear_impl
            }
            
            // 内部实现函数（带记忆化）
            fn #inner_name(#fn_inputs) #fn_output {
                let cache_key = #key_tuple;
                
                // 检查缓存
                #cache_check
                
                // 原始实现
                let result = #fn_block;
                
                // 存入缓存
                #cache_insert
                
                result
            }
            
            // 外部包装函数
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
                let result = #inner_name(#(#call_args),*);
                #clear_name();
                result
        }
        });
    }
    
    let expanded = quote! {
        #(#output)*
    };
    
    expanded.into()
}