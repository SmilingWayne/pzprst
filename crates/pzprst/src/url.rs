//! URL wrapper parsing and generation (port of pzprjs `parser.js` URLData, pzprv3 type).

use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::jsnum;
use crate::variety;

/// Largest accepted `cols` / `rows`.
pub const MAX_DIM: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlParts {
    /// Resolved pzprjs pid.
    pub pid: &'static str,
    /// `editor` / `player` when given via `_edit` / `_play` or `type=`.
    pub mode: Option<String>,
    pub variant: Option<String>,
    pub pflag: String,
    pub cols: usize,
    pub rows: usize,
    pub body: String,
    /// `key=value` query parameters preceding the puzzle data, in order.
    pub params: Vec<(String, String)>,
}

impl UrlParts {
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

pub fn parse(url: &str, diags: &mut Diagnostics) -> Result<UrlParts> {
    let url: String = url.chars().filter(|c| *c != '\r' && *c != '\n').collect();
    let url = url.trim();
    if url.contains("kanpen.net") || url.contains("geocities") || url.contains("indi.s58.xrea.com")
    {
        return Err(Error::UnsupportedUrlType(url.into()));
    }

    let mut rest = match url.find('?') {
        Some(i) => &url[i + 1..],
        None => url,
    };
    if let Some(i) = rest.find('#') {
        rest = &rest[..i];
    }

    // pzprjs skips leading `\w+=\w+&` pairs; base64url states contain `-`, so
    // accept any `key=value&` that ends before the first `/`.
    let mut params = Vec::new();
    loop {
        let slash = rest.find('/').unwrap_or(rest.len());
        let Some(amp) = rest[..slash].find('&') else {
            break;
        };
        let pair = &rest[..amp];
        let Some(eq) = pair.find('=') else { break };
        params.push((pair[..eq].to_string(), pair[eq + 1..].to_string()));
        rest = &rest[amp + 1..];
    }
    let mut mode = None;
    for (k, v) in &params {
        match k.as_str() {
            "type" => mode = Some(v.clone()),
            "state" | "trace" => {}
            _ => diags.push(
                DiagnosticCode::UnknownParameter,
                format!("ignored URL parameter {k:?}"),
            ),
        }
    }
    let (first, qdata) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i + 1..]),
        None => (rest, ""),
    };
    if first.starts_with("v:") {
        return Err(Error::UnsupportedUrlType(
            "variant before puzzle id is only used by file URLs".into(),
        ));
    }
    if is_pzprv_file(first) {
        return Err(Error::UnsupportedUrlType("pzprv3 file URL".into()));
    }
    let mut name = first;
    if let Some(n) = name.strip_suffix("_edit") {
        name = n;
        mode = Some("editor".into());
    } else if let Some(n) = name.strip_suffix("_play") {
        name = n;
        mode = Some("player".into());
    }
    if name.is_empty() {
        return Err(Error::InvalidUrl(format!("no puzzle id in {url:?}")));
    }
    let pid = variety::to_pid(name).ok_or_else(|| Error::UnknownPuzzle(name.into()))?;

    let mut inp: Vec<&str> = qdata.split('/').collect();
    let mut variant = None;
    if let Some(v) = inp.first().and_then(|s| s.strip_prefix("v:")) {
        variant = Some(v.to_string());
        inp.remove(0);
    }
    let mut pflag = String::new();
    if let Some(first) = inp.first() {
        if !first.is_empty() && jsnum::is_nan(first) {
            pflag = first.to_string();
            inp.remove(0);
        }
    }
    let cols = take_dim(&mut inp, "cols")?;
    let rows = take_dim(&mut inp, "rows")?;
    if inp.last().is_some_and(|s| s.is_empty()) {
        inp.pop();
    }
    Ok(UrlParts {
        pid,
        mode,
        variant,
        pflag,
        cols,
        rows,
        body: inp.join("/"),
        params,
    })
}

fn is_pzprv_file(first: &str) -> bool {
    first
        .strip_prefix("pzprv")
        .is_some_and(|r| r.chars().all(|c| c.is_ascii_digit() || c == '.'))
}

fn take_dim(inp: &mut Vec<&str>, what: &str) -> Result<usize> {
    if inp.is_empty() {
        return Err(Error::InvalidSize(format!("missing {what}")));
    }
    let s = inp.remove(0);
    match jsnum::integer(s) {
        Some(n) if n >= 1 && n as usize <= MAX_DIM => Ok(n as usize),
        _ => Err(Error::InvalidSize(format!("{what} = {s:?}"))),
    }
}

/// `outputURLData` with the pid prefix: `pid/[v:variant/][pflag/]cols/rows/body`.
pub fn generate(
    urlid: &str,
    variant: Option<&str>,
    pflag: &str,
    cols: usize,
    rows: usize,
    body: &str,
) -> String {
    let mut out: Vec<String> = Vec::new();
    if let Some(v) = variant {
        out.push(format!("v:{v}"));
    }
    if !pflag.is_empty() {
        out.push(pflag.to_string());
    }
    out.push(cols.to_string());
    out.push(rows.to_string());
    out.push(body.to_string());
    let mut data = out.join("/");
    if !data
        .chars()
        .last()
        .is_some_and(|c| c.is_ascii_alphanumeric())
    {
        data.push('/');
    }
    format!("{urlid}/{data}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(url: &str) -> UrlParts {
        parse(url, &mut Diagnostics::default()).unwrap()
    }

    #[test]
    fn hosts_and_params() {
        let u = p("https://puzz.link/p?slitherlink_edit/3/2/h17d");
        assert_eq!(
            (u.pid, u.cols, u.rows, u.body.as_str()),
            ("slither", 3, 2, "h17d")
        );
        assert_eq!(u.mode.as_deref(), Some("editor"));
        let u = p("https://pzpr-trace.net/p?state=a-b_c&mashu/f/6/6/0000");
        assert_eq!(u.param("state"), Some("a-b_c"));
        assert_eq!(u.pflag, "f");
        let u = p("slither/v:x/f/3/2/h17d/");
        assert_eq!(u.variant.as_deref(), Some("x"));
        assert_eq!(u.body, "h17d");
    }

    #[test]
    fn bad_sizes() {
        let mut d = Diagnostics::default();
        assert!(matches!(
            parse("slither/0/2/0", &mut d),
            Err(Error::InvalidSize(_))
        ));
        assert!(matches!(
            parse("slither", &mut d),
            Err(Error::InvalidSize(_))
        ));
        assert!(matches!(
            parse("nonexistent/1/1", &mut d),
            Err(Error::UnknownPuzzle(_))
        ));
    }

    #[test]
    fn generate_appends_slash() {
        assert_eq!(generate("slither", None, "", 1, 1, "."), "slither/1/1/./");
        assert_eq!(generate("mashu", None, "f", 6, 6, "0"), "mashu/f/6/6/0");
    }
}
