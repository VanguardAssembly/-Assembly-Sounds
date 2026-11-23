//! Source texts (1.9): the scripture, treatise, manual, book or study each session comes from,
//! so you can read about, and read in English, the text behind the practice.
//!
//! The library is kept as plain text in `src/texts/*.txt` and linked to the sessions by
//! `src/texts/map.txt`. The format is described at the top of `src/texts/map.txt`.
//!
//! Translations are Assembly Sounds' own plain-English renderings of the original texts unless
//! a public-domain translation is credited. Modern works still under copyright are summarised in
//! our own words, never reproduced; each entry lists published translations for further reading.

use crate::plan::{Preset, TextRef};
use serde::Serialize;
use std::sync::OnceLock;

const FILES: &[(&str, &str)] = &[
    ("bible.txt", include_str!("texts/bible.txt")),
    ("christian1.txt", include_str!("texts/christian1.txt")),
    ("christian2.txt", include_str!("texts/christian2.txt")),
    ("rosicrucian.txt", include_str!("texts/rosicrucian.txt")),
    ("jewish.txt", include_str!("texts/jewish.txt")),
    ("islamic.txt", include_str!("texts/islamic.txt")),
    ("hindu1.txt", include_str!("texts/hindu1.txt")),
    ("hindu2.txt", include_str!("texts/hindu2.txt")),
    ("jain_sikh.txt", include_str!("texts/jain_sikh.txt")),
    ("theravada.txt", include_str!("texts/theravada.txt")),
    ("tibetan1.txt", include_str!("texts/tibetan1.txt")),
    ("tibetan2.txt", include_str!("texts/tibetan2.txt")),
    ("zen.txt", include_str!("texts/zen.txt")),
    ("eastasian.txt", include_str!("texts/eastasian.txt")),
    ("taoist.txt", include_str!("texts/taoist.txt")),
    ("shinto_zoroastrian_shamanic.txt", include_str!("texts/shinto_zoroastrian_shamanic.txt")),
    ("western.txt", include_str!("texts/western.txt")),
    ("classical_celtic.txt", include_str!("texts/classical_celtic.txt")),
    ("hypnosis.txt", include_str!("texts/hypnosis.txt")),
    ("frontier.txt", include_str!("texts/frontier.txt")),
    ("science1.txt", include_str!("texts/science1.txt")),
    ("science2.txt", include_str!("texts/science2.txt")),
    ("science3.txt", include_str!("texts/science3.txt")),
    ("science4.txt", include_str!("texts/science4.txt")),
    ("sound.txt", include_str!("texts/sound.txt")),
];
const MAP: &str = include_str!("texts/map.txt");

/// Library groups, in the order the Library screen shows them.
pub const FAMS: &[&str] = &[
    "christian", "rosicrucian", "masonic", "jewish", "islamic", "bahai", "hindu", "kundalini", "jain", "sikh",
    "theravada", "tibetan", "zen", "eastasian", "taoist", "shinto", "zoroastrian", "shamanic", "western",
    "classical", "celtic", "hypnosis", "frontier", "science", "sound",
];
pub const KINDS: &[&str] = &[
    "scripture", "sutra", "treatise", "manual", "poem", "prayer", "teaching", "liturgy", "ritual", "book",
    "study", "review", "report", "history",
];

/// One text in the library, as sent to the UI.
#[derive(Serialize, Clone, Debug, Default)]
pub struct Text {
    pub id: String,
    pub title: String,
    /// Original title, language or script.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub orig: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub by: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub date: String,
    pub fam: String,
    pub kind: String,
    /// Full reference for a study or book (trusted HTML).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cite: String,
    /// What the text is and how the practice comes from it (trusted HTML).
    pub about: String,
    /// The passages in English: heading + trusted HTML.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
    /// The parts hold the complete text, not a selection.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub full: bool,
    /// Who translated or rendered the passages.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub credit: String,
    /// Published translations and studies for further reading (trusted HTML).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub more: Vec<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Part {
    pub h: String,
    pub html: String,
}

/// The whole library (parsed once).
pub fn library() -> &'static [Text] {
    static LIB: OnceLock<Vec<Text>> = OnceLock::new();
    LIB.get_or_init(|| {
        let mut v = Vec::new();
        for (name, src) in FILES {
            if let Err(e) = parse(name, src, &mut v) {
                eprintln!("texts: {e}");
            }
        }
        v
    })
}

/// Fill in `p.texts` for every session from map.txt (lines are applied in file order; a text
/// is listed once, with the first note given for it).
pub fn link(v: &mut [Preset]) {
    let rules = parse_map(MAP).unwrap_or_else(|e| {
        eprintln!("texts: {e}");
        Vec::new()
    });
    for p in v.iter_mut() {
        let mut out: Vec<TextRef> = Vec::new();
        for (pats, items) in &rules {
            if !pats.iter().any(|pat| matches(pat, &p.id)) {
                continue;
            }
            for (id, note) in items {
                if let Some(r) = out.iter_mut().find(|r| &r.id == id) {
                    if r.note.is_empty() && !note.is_empty() {
                        r.note = inline(note);
                    }
                } else {
                    out.push(TextRef { id: id.clone(), note: inline(note) });
                }
            }
        }
        p.texts = out;
    }
}

fn matches(pat: &str, id: &str) -> bool {
    match pat.strip_suffix('*') {
        Some(pre) => id.starts_with(pre),
        None => pat == id,
    }
}

type Rules = Vec<(Vec<String>, Vec<(String, String)>)>;

fn parse_map(src: &str) -> Result<Rules, String> {
    let mut rules = Vec::new();
    for (n, line) in src.lines().enumerate() {
        let l = line.trim();
        if l.is_empty() || l.starts_with("//") {
            continue;
        }
        let (left, right) = l.split_once(':').ok_or_else(|| format!("map.txt:{}: missing ':'", n + 1))?;
        let pats: Vec<String> = left.split_whitespace().map(String::from).collect();
        if pats.is_empty() {
            return Err(format!("map.txt:{}: no session ids", n + 1));
        }
        let mut items = Vec::new();
        for it in right.split('|') {
            let it = it.trim();
            if it.is_empty() {
                continue;
            }
            let (id, note) = match it.split_once('=') {
                Some((a, b)) => (a.trim(), b.trim()),
                None => (it, ""),
            };
            if id.is_empty() || id.contains(' ') {
                return Err(format!("map.txt:{}: bad text id '{id}'", n + 1));
            }
            items.push((id.to_string(), note.to_string()));
        }
        rules.push((pats, items));
    }
    Ok(rules)
}

// ------------------------------------------------------------------ the text format

fn parse(name: &str, src: &str, out: &mut Vec<Text>) -> Result<(), String> {
    #[derive(PartialEq)]
    enum Mode { Head, About, Part }
    let mut cur: Option<Text> = None;
    let mut mode = Mode::Head;
    let mut head = String::new();
    let mut buf: Vec<String> = Vec::new();

    fn flush(t: &mut Text, mode: &Mode, head: &str, buf: &mut Vec<String>) {
        let lines: Vec<&str> = buf.iter().map(|s| s.as_str()).collect();
        match mode {
            Mode::About => t.about = body(&lines),
            Mode::Part => t.parts.push(Part { h: inline(head), html: body(&lines) }),
            Mode::Head => {}
        }
        buf.clear();
    }

    for (n, raw) in src.lines().enumerate() {
        let l = raw.trim_end();
        let at = || format!("{name}:{}", n + 1);
        if l.starts_with("//") {
            continue;
        }
        if let Some(id) = l.strip_prefix("@ ") {
            if let Some(mut t) = cur.take() {
                flush(&mut t, &mode, &head, &mut buf);
                check(&t, name)?;
                out.push(t);
            }
            cur = Some(Text { id: id.trim().to_string(), ..Default::default() });
            mode = Mode::Head;
            continue;
        }
        let Some(t) = cur.as_mut() else {
            if l.trim().is_empty() {
                continue;
            }
            return Err(format!("{}: text before the first '@ id'", at()));
        };
        if let Some(h) = l.strip_prefix("## ") {
            flush(t, &mode, &head, &mut buf);
            head = h.trim().to_string();
            mode = Mode::Part;
            continue;
        }
        if mode != Mode::Head {
            buf.push(l.to_string());
            continue;
        }
        if l.trim().is_empty() {
            continue;
        }
        if l == "about:" {
            mode = Mode::About;
            continue;
        }
        let (k, v) = l.split_once(": ").ok_or_else(|| format!("{}: expected 'key: value'", at()))?;
        let v = v.trim();
        match k {
            "title" => t.title = v.into(),
            "orig" => t.orig = v.into(),
            "by" => t.by = v.into(),
            "date" => t.date = v.into(),
            "fam" => t.fam = v.into(),
            "kind" => t.kind = v.into(),
            "cite" => t.cite = inline(v),
            "credit" => t.credit = inline(v),
            "more" => t.more.push(inline(v)),
            "full" => t.full = v == "yes",
            _ => return Err(format!("{}: unknown key '{k}'", at())),
        }
    }
    if let Some(mut t) = cur.take() {
        flush(&mut t, &mode, &head, &mut buf);
        check(&t, name)?;
        out.push(t);
    }
    Ok(())
}

fn check(t: &Text, name: &str) -> Result<(), String> {
    let e = |m: &str| Err(format!("{name}: {}: {m}", t.id));
    if t.title.is_empty() { return e("no title"); }
    if t.about.is_empty() { return e("no about"); }
    if !FAMS.contains(&t.fam.as_str()) { return e("unknown fam"); }
    if !KINDS.contains(&t.kind.as_str()) { return e("unknown kind"); }
    Ok(())
}

/// Body text to HTML. Blocks are separated by blank lines. Within a block, runs of lines are:
/// `> ` verse (line breaks kept), `- ` bullet list, `1. ` numbered list, `= ` original-language
/// lines (shown in italics), anything else a paragraph (lines joined with spaces).
fn body(lines: &[&str]) -> String {
    let mut html = String::new();
    let mut block: Vec<&str> = Vec::new();
    for l in lines.iter().copied().chain(std::iter::once("")) {
        if l.trim().is_empty() {
            emit(&block, &mut html);
            block.clear();
        } else {
            block.push(l);
        }
    }
    html
}

#[derive(PartialEq, Clone, Copy)]
enum Run { Para, Verse, List, Num, Orig }

fn run_of(l: &str) -> Run {
    if l.starts_with("> ") || l == ">" {
        Run::Verse
    } else if l.starts_with("- ") {
        Run::List
    } else if l.starts_with("= ") {
        Run::Orig
    } else if num_prefix(l).is_some() {
        Run::Num
    } else {
        Run::Para
    }
}

/// "12. text" -> (12, "text")
fn num_prefix(l: &str) -> Option<(u32, &str)> {
    let d = l.find(". ")?;
    let n: u32 = l[..d].parse().ok()?;
    if d > 3 { return None; }
    Some((n, &l[d + 2..]))
}

fn emit(block: &[&str], out: &mut String) {
    let mut i = 0;
    while i < block.len() {
        let r = run_of(block[i]);
        let mut j = i;
        while j < block.len() && run_of(block[j]) == r {
            j += 1;
        }
        let run = &block[i..j];
        match r {
            Run::Para => {
                out.push_str("<p>");
                out.push_str(&inline(&run.iter().map(|s| s.trim()).collect::<Vec<_>>().join(" ")));
                out.push_str("</p>");
            }
            Run::Verse => {
                let ls: Vec<String> = run.iter().map(|s| inline(s.trim_start_matches('>').trim())).collect();
                out.push_str("<p class=\"vs\">");
                out.push_str(&ls.join("<br>"));
                out.push_str("</p>");
            }
            Run::Orig => {
                let ls: Vec<String> = run.iter().map(|s| inline(s[2..].trim())).collect();
                out.push_str("<p class=\"og\">");
                out.push_str(&ls.join("<br>"));
                out.push_str("</p>");
            }
            Run::List => {
                out.push_str("<ul>");
                for s in run {
                    out.push_str("<li>");
                    out.push_str(&inline(s[2..].trim()));
                    out.push_str("</li>");
                }
                out.push_str("</ul>");
            }
            Run::Num => {
                out.push_str("<ol>");
                for s in run {
                    let (n, t) = num_prefix(s).unwrap_or((0, s));
                    out.push_str(&format!("<li value=\"{n}\">"));
                    out.push_str(&inline(t.trim()));
                    out.push_str("</li>");
                }
                out.push_str("</ol>");
            }
        }
        i = j;
    }
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Escape, then **bold** and *italic*.
pub fn inline(s: &str) -> String {
    pairs(&pairs(&esc(s), "**", "<b>", "</b>"), "*", "<i>", "</i>")
}

fn pairs(s: &str, m: &str, open: &str, close: &str) -> String {
    let parts: Vec<&str> = s.split(m).collect();
    let marks = parts.len() - 1;
    if marks < 2 {
        return s.to_string();
    }
    let usable = marks - marks % 2;
    let mut out = String::new();
    for (k, p) in parts.iter().enumerate() {
        if k > 0 {
            if k <= usable {
                out.push_str(if k % 2 == 1 { open } else { close });
            } else {
                out.push_str(m);
            }
        }
        out.push_str(p);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn every_file_parses() {
        let mut v = Vec::new();
        for (name, src) in FILES {
            parse(name, src, &mut v).unwrap();
        }
        let mut seen = HashSet::new();
        for t in &v {
            assert!(seen.insert(t.id.clone()), "duplicate text id {}", t.id);
            assert!(!t.title.contains('*'), "{}: markup in title", t.id);
        }
        parse_map(MAP).unwrap();
    }

    #[test]
    fn every_session_has_its_source_texts() {
        let lib: HashMap<&str, &Text> = library().iter().map(|t| (t.id.as_str(), t)).collect();
        let v = crate::presets::all();
        let mut used = HashSet::new();
        for p in &v {
            assert!(!p.texts.is_empty(), "{} has no source text", p.id);
            for r in &p.texts {
                assert!(lib.contains_key(r.id.as_str()), "{} links to unknown text {}", p.id, r.id);
                used.insert(r.id.clone());
            }
        }
        for t in library() {
            assert!(used.contains(&t.id), "text {} is not used by any session", t.id);
        }
        // every pattern in map.txt matches at least one session (catches typos in session ids)
        for (pats, _) in parse_map(MAP).unwrap() {
            for pat in pats {
                assert!(v.iter().any(|p| matches(&pat, &p.id)), "map.txt: '{pat}' matches no session");
            }
        }
    }

    #[test]
    fn markup() {
        assert_eq!(inline("a *b* **c** <d>"), "a <i>b</i> <b>c</b> &lt;d&gt;");
        assert_eq!(inline("5 * 3"), "5 * 3");
        assert_eq!(body(&["> one", "> two", "", "- x", "- y", "", "1. a", "2. b", "", "= Om", "", "p q", "r"]),
            "<p class=\"vs\">one<br>two</p><ul><li>x</li><li>y</li></ul><ol><li value=\"1\">a</li><li value=\"2\">b</li></ol><p class=\"og\">Om</p><p>p q r</p>");
    }
}
