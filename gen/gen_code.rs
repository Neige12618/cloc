use super::language::Languages;
use quote::{format_ident, quote};

pub fn generate_code(data: Languages) -> String {
    let data = data.languages;

    let name = remap(&data, |l| format_ident!("{}", l.name));

    let enm = quote! {
        #[derive(PartialEq, Clone, Copy, clap::ValueEnum)]
        pub enum LanguageType {
            #(#name,)*
        }
    };

    let re_name = remap(&data, |l| {
        format_ident!("{}_QUOTE_RE", l.name.to_uppercase())
    });

    let re = quote! {
        #(static #re_name: once_cell::sync::Lazy<Vec<regex::Regex>> = once_cell::sync::Lazy::new(|| {
            LanguageType::#name
                .quotes()
                .iter()
                .map(|q| regex::Regex::new(&format!("{}(.*?){}", q.0, q.1)).unwrap())
                .collect()
        });)*
    };

    let line_comment = remap(&data, |l| {
        let lc = &l.line_comment;
        quote! {
            &[#(#lc)*]
        }
    });

    let lc = quote! {
        pub fn line_comment(&self) -> &'static [StaticStr] {
            match self {
                #(LanguageType::#name => #line_comment,)*
            }
        }
    };

    let tuple_quote = |l: &Vec<(String, String)>| {
        remap(&l, |l| {
            let (l1, l2) = (&l.0, &l.1);
            quote! {
                (#l1, #l2),
            }
        })
    };

    let multil_line = remap(&data, |l| {
        let mlc = tuple_quote(&l.multi_line_comments);
        quote! {
            &[#(#mlc)*]
        }
    });

    let mlc = quote! {
        pub fn multi_line_comments(&self) -> &'static [(StaticStr, StaticStr)] {
            match self {
                #(LanguageType::#name => #multil_line,)*
            }
        }
    };

    let quotes = remap(&data, |l| {
        let mlc = tuple_quote(&l.quotes);
        quote! {
            &[#(#mlc)*]
        }
    });
    let quotes = quote! {
        pub fn quotes(&self) -> &'static [(StaticStr, StaticStr)] {
            match self {
                #(LanguageType::#name => #quotes,)*
            }
        }
    };

    let qe = quote! {
        pub fn quotes_regex(&self) -> &'static Vec<regex::Regex> {
            match self {
                #(LanguageType::#name => &#re_name,)*
            }
        }
    };

    let vq = remap(&data, |l| {
        let vq = tuple_quote(&l.verbatim_quotes);
        quote! {
            &[#(#vq)*]
        }
    });

    let vq = quote! {
        pub fn verbatim_quotes(&self) -> &'static [(StaticStr, StaticStr)] {
            match self {
                #(LanguageType::#name => #vq,)*
            }
        }
    };

    let ffe = remap(&data, |l| {
        let e = &l.extensions;
        quote! {
            #(#e)|*
        }
    });

    let ffe = quote! {
        pub fn from_file_extension(extension: &str) -> Option<Self> {
            match extension {
                #(#ffe => Some(LanguageType::#name),)*
                _ => None,
            }
        }
    };

    let result = quote! {
        #enm
        #re

        type StaticStr = &'static str;

        impl LanguageType {
            #lc
            #mlc
            #quotes
            #qe
            #vq
            #ffe
        }

    };
    result.to_string()
}

fn remap<T, B, F>(v: &Vec<T>, f: F) -> Vec<B>
where
    F: Fn(&T) -> B,
{
    v.iter().map(f).collect()
}
