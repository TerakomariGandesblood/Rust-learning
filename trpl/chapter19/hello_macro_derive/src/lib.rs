use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

// proc_macro crate 是编译器用来读取和操作我们 Rust 代码的 API
// syn crate 将字符串中的 Rust 代码解析成为一个可以操作的数据结构
// quote 则将 syn 解析的数据结构转换回 Rust 代码
// 当用户在一个类型上指定 #[derive(HelloMacro)] 时，hello_macro_derive 函数将会被调用
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 在错误时 panic 对过程宏来说是必须的，因为 proc_macro_derive 函数必须返回 TokenStream 而不是 Result
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // quote! 宏能让我们编写希望返回的 Rust 代码
    let code = quote! {
        // 使用 #name，quote! 会以名为 name 的变量值来替换它
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 接收一个 Rust 表达式，然后在编译时将表达式转换为一个字符串字面量
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // 转换类型
    code.into()
}
