// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::token_stream::IntoIter as ProcIter;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::Peekable;

#[derive(Clone, Copy, Debug)]
enum BorrowKind {
    Weak,
    WeakAllowNone,
    Strong,
}

impl BorrowKind {
    fn to_str(self) -> &'static str {
        match self {
            Self::Weak => "@weak",
            Self::WeakAllowNone => "@weak-allow-none",
            Self::Strong => "@strong",
        }
    }
}

enum WrapperKind {
    DefaultPanic,
    DefaultReturn(String),
}

impl WrapperKind {
    fn to_str(&self) -> String {
        match *self {
            Self::DefaultPanic => "@default-panic".to_owned(),
            Self::DefaultReturn(ref r) => format!("@default-return {}", r),
        }
    }

    fn keyword(&self) -> &'static str {
        match *self {
            Self::DefaultPanic => "default-panic",
            Self::DefaultReturn(_) => "default-return",
        }
    }
}

#[derive(Debug)]
struct ElemToClone {
    name: String,
    alias: Option<String>,
    borrow_kind: BorrowKind,
}

impl ElemToClone {
    fn to_str_before(&self) -> String {
        match self.borrow_kind {
            BorrowKind::Weak | BorrowKind::WeakAllowNone => format!(
                "let {} = {}::clone::Downgrade::downgrade(&{});",
                if let Some(ref a) = self.alias {
                    a
                } else {
                    &self.name
                },
                crate_ident_new(),
                self.name,
            ),
            BorrowKind::Strong => format!(
                "let {} = {}.clone();",
                if let Some(ref a) = self.alias {
                    a
                } else {
                    &self.name
                },
                self.name,
            ),
        }
    }

    fn to_str_after(&self, wrapper_kind: &WrapperKind) -> String {
        match (self.borrow_kind, wrapper_kind) {
            (BorrowKind::Weak, WrapperKind::DefaultPanic) => {
                let name = if let Some(ref a) = self.alias {
                    a
                } else {
                    &self.name
                };
                format!(
                    "\
let {0} = match {1}::clone::Upgrade::upgrade(&{0}) {{
    Some(val) => val,
    None => panic!(
        \"failed to upgrade `{0}` (if you don't want to panic, use @default-return)\",
    ),
}};",
                    name,
                    crate_ident_new(),
                )
            }
            (BorrowKind::Weak, WrapperKind::DefaultReturn(ref r)) => {
                let name = if let Some(ref a) = self.alias {
                    a
                } else {
                    &self.name
                };
                format!(
                    "\
let {0} = match {1}::clone::Upgrade::upgrade(&{0}) {{
    Some(val) => val,
    None => {{
        {1}::g_debug!(
            {1}::CLONE_MACRO_LOG_DOMAIN,
            \"Failed to upgrade {0}\",
        );
        let ___return_value = || {{ {2} }};
        return ___return_value();
    }}
}};",
                    name,
                    crate_ident_new(),
                    r,
                )
            }
            (BorrowKind::WeakAllowNone, _) => format!(
                "let {0} = {1}::clone::Upgrade::upgrade(&{0});",
                if let Some(ref a) = self.alias {
                    a
                } else {
                    &self.name
                },
                crate_ident_new(),
            ),
            _ => String::new(),
        }
    }
}

enum SimpleToken {
    Punct(&'static str),
    Ident(&'static str),
}

impl SimpleToken {
    fn to_str(&self) -> &str {
        match *self {
            Self::Punct(p) => p,
            Self::Ident(i) => i,
        }
    }
}

impl PartialEq<TokenTree> for SimpleToken {
    fn eq(&self, other: &TokenTree) -> bool {
        match (self, other) {
            (SimpleToken::Punct(ref p1), TokenTree::Punct(ref p2)) => *p1 == p2.to_string(),
            (SimpleToken::Ident(ref i1), TokenTree::Ident(ref i2)) => *i1 == i2.to_string(),
            _ => false,
        }
    }
}

fn is_punct(elem: &TokenTree, punct: &str) -> bool {
    match elem {
        TokenTree::Punct(ref p) => p.to_string() == punct,
        _ => false,
    }
}

enum TokenCheck {
    UnexpectedToken(String, String),
    UnexpectedEnd(String),
}

fn check_tokens(
    tokens_to_check: &[SimpleToken],
    parts: &mut Peekable<ProcIter>,
) -> Result<(), TokenCheck> {
    let mut tokens = String::new();

    for token in tokens_to_check {
        if let Some(next) = parts.next() {
            if *token != next {
                return Err(TokenCheck::UnexpectedToken(
                    tokens,
                    token.to_str().to_owned(),
                ));
            }
            tokens.push_str(token.to_str());
        } else {
            return Err(TokenCheck::UnexpectedEnd(tokens));
        }
    }
    Ok(())
}

fn get_full_ident(parts: &mut Peekable<ProcIter>, borrow_kind: BorrowKind) -> String {
    let mut name = String::new();
    let mut prev_is_ident = false;

    loop {
        match parts.peek() {
            Some(TokenTree::Punct(p)) => {
                let p_s = p.to_string();
                if p_s == "," || p_s == "=" {
                    break;
                } else if p_s == "." {
                    if !prev_is_ident {
                        panic!("Unexpected `.` after `{}`", borrow_kind.to_str());
                    }
                    prev_is_ident = false;
                    name.push('.');
                    parts.next();
                } else if name.is_empty() {
                    panic!("Expected ident, found `{}`", p_s);
                } else {
                    panic!("Expected ident, found `{}` after `{}`", p_s, name);
                }
            }
            Some(TokenTree::Ident(i)) => {
                if prev_is_ident {
                    break;
                }
                prev_is_ident = true;
                name.push_str(&i.to_string());
                parts.next();
            }
            Some(x) if name.is_empty() => panic!("Expected ident, found `{}`", x.to_string()),
            Some(x) => panic!("Expected ident, found `{}` after `{}`", x.to_string(), name),
            None => panic!("Unexpected end after ident `{}`", name),
        }
    }
    if name.is_empty() {
        panic!(
            "Expected ident, found `{}`",
            parts.next().unwrap().to_string()
        );
    }
    name
}

fn get_keyword(parts: &mut Peekable<ProcIter>) -> String {
    let mut ret = String::new();
    let mut prev_is_ident = false;
    let mut stored = false;

    loop {
        match parts.peek() {
            Some(TokenTree::Ident(i)) => {
                if prev_is_ident {
                    break;
                }
                prev_is_ident = true;
                if stored {
                    ret.push('-');
                    stored = false;
                }
                ret.push_str(&i.to_string());
            }
            Some(TokenTree::Punct(p)) if p.to_string() == "-" => {
                if !prev_is_ident {
                    break;
                }
                // This is to prevent to push `-` if the next item isn't an ident.
                prev_is_ident = false;
                stored = true;
            }
            _ => break,
        }
        parts.next();
    }
    ret
}

fn parse_ident(parts: &mut Peekable<ProcIter>, elements: &mut Vec<ElemToClone>) {
    let borrow_kind = match get_keyword(parts).as_str() {
        "strong" => BorrowKind::Strong,
        "weak" => BorrowKind::Weak,
        "weak-allow-none" => BorrowKind::WeakAllowNone,
        "default-return" => panic!("`@default-return` should be after `=>`"),
        "default-panic" => panic!("`@default-panic` should be after `=>`"),
        k => panic!(
            "Unknown keyword `{}`, only `weak`, `weak-allow-none` and `strong` are allowed",
            k,
        ),
    };
    let name = get_full_ident(parts, borrow_kind);
    let alias = match parts.peek() {
        Some(TokenTree::Ident(p)) if p.to_string() == "as" => {
            parts.next();
            match parts.next() {
                Some(TokenTree::Ident(i)) => Some(i.to_string()),
                Some(x) => panic!(
                    "Expected ident after `as` keyword, found `{}`",
                    x.to_string()
                ),
                None => panic!("Unexpected end after `as` keyword"),
            }
        }
        Some(TokenTree::Ident(p)) => panic!("Unexpected `{}`", p.to_string()),
        _ => None,
    };
    if name == "self" && alias.is_none() {
        panic!(
            "Can't use `self` as variable name. Try storing it in a temporary variable or \
                rename it using `as`."
        );
    } else if name.ends_with('.') {
        panic!("Invalid variable name: `{}`", name);
    } else if name.contains('.') && alias.is_none() {
        panic!(
            "`{}`: Field accesses are not allowed as is, you must rename it!",
            name
        );
    }
    elements.push(ElemToClone {
        name,
        alias,
        borrow_kind,
    });
}

fn group_to_string(g: &Group) -> String {
    format!(
        "{}{}{}",
        match g.delimiter() {
            Delimiter::Parenthesis => "(",
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::None => "",
        },
        tokens_to_string(g.stream().into_iter().peekable()),
        match g.delimiter() {
            Delimiter::Parenthesis => ")",
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::None => "",
        },
    )
}

fn get_expr(parts: &mut Peekable<ProcIter>) -> String {
    let mut ret = String::new();
    let mut total = 0;
    match parts.next() {
        Some(TokenTree::Literal(l)) => ret.push_str(&l.to_string()),
        Some(TokenTree::Ident(i)) => ret.push_str(&i.to_string()),
        Some(TokenTree::Punct(p)) => match p.to_string().as_str() {
            "[" | "{" | "(" => {
                total += 1;
            }
            x => panic!("Unexpected token `{}` after `@default-return`", x),
        },
        Some(TokenTree::Group(g)) => return group_to_string(&g),
        None => panic!("Unexpected end after `@default-return`"),
    };
    loop {
        match parts.peek() {
            Some(TokenTree::Punct(p)) => {
                let p_s = p.to_string();
                if p_s == "{" || p_s == "(" || p_s == "[" || p_s == "<" {
                    total += 1;
                } else if p_s == "}" || p_s == ")" || p_s == "]" || p_s == ">" {
                    total -= 1;
                } else if p_s == "," && total == 0 {
                    return ret;
                }
                ret.push_str(&p_s);
            }
            Some(TokenTree::Group(g)) => ret.push_str(&group_to_string(g)),
            Some(x) => {
                if total == 0 && !ret.ends_with(':') {
                    return ret;
                }
                ret.push_str(&x.to_string())
            }
            None => panic!("Unexpected end after `{}`", ret),
        }
        parts.next();
    }
}

fn get_return_kind(parts: &mut Peekable<ProcIter>) -> WrapperKind {
    match check_tokens(
        &[SimpleToken::Ident("default"), SimpleToken::Punct("-")],
        parts,
    ) {
        Err(TokenCheck::UnexpectedToken(tokens, unexpected_token)) => {
            panic!("Unknown keyword `{}{}`", tokens, unexpected_token);
        }
        Err(TokenCheck::UnexpectedEnd(tokens)) => {
            panic!("Unexpected end after tokens `{}`", tokens);
        }
        Ok(()) => {}
    }
    match parts.next() {
        Some(TokenTree::Ident(i)) => {
            let i_s = i.to_string();
            if i_s == "panic" {
                return WrapperKind::DefaultPanic;
            } else if i_s != "return" {
                panic!("Unknown keyword `@default-{}`", i_s);
            }
        }
        Some(x) => panic!("Unknown token `{}` after `@default-`", x.to_string()),
        None => panic!("Unexpected end after `@default-`"),
    }
    WrapperKind::DefaultReturn(get_expr(parts))
}

fn parse_return_kind(parts: &mut Peekable<ProcIter>) -> WrapperKind {
    match parts.peek() {
        Some(TokenTree::Punct(p)) if p.to_string() == "@" => {}
        None => panic!("Unexpected end 2"),
        _ => return WrapperKind::DefaultPanic,
    }
    parts.next();
    let ret = get_return_kind(parts);
    match check_tokens(&[SimpleToken::Punct(",")], parts) {
        Err(TokenCheck::UnexpectedToken(_, unexpected_token)) => {
            panic!(
                "Expected `,` after `{}`, found `{}`",
                ret.to_str(),
                unexpected_token
            );
        }
        Err(TokenCheck::UnexpectedEnd(tokens)) => {
            panic!("Expected `,` after `{}{}`", ret.to_str(), tokens);
        }
        Ok(()) => {}
    }
    ret
}

fn check_before_closure(parts: &mut Peekable<ProcIter>) {
    match parts.peek() {
        Some(TokenTree::Ident(i)) if i.to_string() == "move" => {}
        Some(TokenTree::Ident(i)) if i.to_string() == "async" => {
            panic!("async blocks are not supported by the clone! macro");
        }
        Some(TokenTree::Ident(i)) if i.to_string() == "default" => {
            let ret = get_return_kind(parts);
            panic!("Missing `@` before `{}`", ret.keyword());
        }
        Some(TokenTree::Punct(p)) if p.to_string() == "|" => {
            panic!("Closure needs to be \"moved\" so please add `move` before closure")
        }
        _ => panic!("Missing `move` and closure declaration"),
    }
    parts.next();
    match parts.next() {
        Some(TokenTree::Punct(p)) if p.to_string() == "|" => {}
        Some(x) => panic!("Expected closure, found `{}`", x.to_string()),
        None => panic!("Expected closure"),
    }
}

fn get_closure(parts: &mut Peekable<ProcIter>) -> Vec<TokenTree> {
    let mut ret = Vec::new();

    loop {
        match parts.next() {
            Some(TokenTree::Punct(p)) if p.to_string() == "|" => break,
            Some(x) => ret.push(x),
            None => panic!("Unexpected end 3"),
        }
    }
    ret
}

pub fn tokens_to_string(parts: Peekable<ProcIter>) -> String {
    let mut ret = String::new();
    // This is used in case of "if ident" or other similar cases.
    let mut prev_is_ident = false;
    let handle_ident_like = |i: String, ret: &mut String, prev_is_ident: &mut bool| {
        if *prev_is_ident {
            ret.push(' ');
        }
        ret.push_str(&i);
        *prev_is_ident = true;
    };

    for token in parts {
        match token {
            TokenTree::Punct(p) => {
                prev_is_ident = false;
                ret.push_str(&p.to_string());
            }
            TokenTree::Ident(i) => handle_ident_like(i.to_string(), &mut ret, &mut prev_is_ident),
            TokenTree::Literal(l) => handle_ident_like(l.to_string(), &mut ret, &mut prev_is_ident),
            TokenTree::Group(g) => {
                prev_is_ident = false;
                ret.push_str(&group_to_string(&g));
            }
        }
    }
    ret
}

pub(crate) fn clone_inner(item: TokenStream) -> TokenStream {
    let mut parts = item.into_iter().peekable();
    let mut elements = Vec::new();
    let mut prev_is_ident = false;

    loop {
        match parts.next() {
            Some(TokenTree::Punct(ref p)) => {
                let p_s = p.to_string();
                if p_s == "=" && parts.peek().map_or_else(|| false, |n| is_punct(n, ">")) {
                    parts.next();
                    break;
                } else if p_s == "@" {
                    parse_ident(&mut parts, &mut elements);
                    prev_is_ident = true;
                } else if p_s == "," {
                    if !prev_is_ident {
                        panic!("Unexpected `,`");
                    }
                    prev_is_ident = false;
                } else if p_s == "|" {
                    if elements.is_empty() {
                        panic!("If you have nothing to clone, no need to use this macro!");
                    } else {
                        panic!("Expected `=>` before closure");
                    }
                }
            }
            Some(TokenTree::Ident(i)) => {
                panic!(
                    "Unexpected ident `{}`: you need to specify if this is a weak or a strong \
                     clone.",
                    i.to_string(),
                );
            }
            Some(t) => panic!("Unexpected token `{}`", t.to_string()),
            None => panic!("Unexpected end 4"),
        }
    }
    if elements.is_empty() {
        panic!("If you have nothing to clone, no need to use this macro!");
    }
    let return_kind = parse_return_kind(&mut parts);
    check_before_closure(&mut parts);
    let closure = get_closure(&mut parts);
    let mut body = TokenStream::new();
    body.extend(parts.collect::<Vec<_>>());

    // To prevent to lose the spans in case some errors occur in the code, we need to keep `body`!
    //
    // If we replaced everything that follows with a `format!`, it'd look like this:
    //
    // format!(
    //     "{{\n{}\nmove |{}| {{\n{}\nlet ____ret = {{ {} }};\n____ret\n}}\n}}",
    //     elements
    //         .iter()
    //         .map(|x| x.to_str_before())
    //         .collect::<Vec<_>>()
    //         .join("\n"),
    //     closure,
    //     elements
    //         .iter()
    //         .map(|x| x.to_str_after(&return_kind))
    //         .collect::<Vec<_>>()
    //         .join("\n"),
    //     body,
    // )
    let mut ret: Vec<TokenTree> = vec![];
    for el in &elements {
        let stream: TokenStream = el
            .to_str_before()
            .parse()
            .expect("failed to convert element");
        ret.extend(stream.into_iter().collect::<Vec<_>>());
    }
    ret.extend(vec![
        TokenTree::Ident(Ident::new("move", Span::call_site())),
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
    ]);
    ret.extend(closure);
    ret.extend(vec![TokenTree::Punct(Punct::new('|', Spacing::Alone))]);
    let mut inner: Vec<TokenTree> = Vec::new();
    for el in elements {
        let stream: TokenStream = el
            .to_str_after(&return_kind)
            .parse()
            .expect("failed to convert element after");
        inner.extend(stream.into_iter().collect::<Vec<_>>());
    }
    // The commented lines that follow *might* be useful, don't know. Just in case, I'm keeping
    // them around. You're welcome future me!
    inner.extend(vec![
        // TokenTree::Ident(Ident::new("let", Span::call_site())),
        // TokenTree::Ident(Ident::new("____ret", Span::call_site())),
        // TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Brace, body)),
        // TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        // TokenTree::Ident(Ident::new("____ret", Span::call_site())),
    ]);
    let mut inners = TokenStream::new();
    inners.extend(inner);
    ret.extend(vec![TokenTree::Group(Group::new(Delimiter::Brace, inners))]);

    let mut rets = TokenStream::new();
    rets.extend(ret);

    TokenTree::Group(Group::new(Delimiter::Brace, rets)).into()
}
