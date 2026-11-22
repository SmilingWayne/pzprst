//! Spec examples, diagnostics, intentional divergences, states and trace URLs.

use pzprst::{canonical_url, Board, DiagnosticCode, Error, Options, Transform};

fn opts() -> Options {
    Options::default()
}

fn strict() -> Options {
    Options {
        strict: true,
        ..Options::default()
    }
}

fn codes(input: &str) -> Vec<DiagnosticCode> {
    Board::from_url(input, &opts())
        .unwrap()
        .diagnostics()
        .iter()
        .map(|d| d.code)
        .collect()
}

#[test]
fn spec_examples() {
    for (input, want) in [
        ("slither/3/2/h12g3", "slither/3/2/h17d"),
        (
            "https://puzz.link/p?slitherlink/3/2/h17d",
            "slither/3/2/h17d",
        ),
        ("slither/3/2/h17d/", "slither/3/2/h17d"),
        ("slither/3/2/h17dzzz", "slither/3/2/h17d"),
        ("masyu/6/6/00016f769000", "mashu/6/6/00016f769000"),
        ("pearl/6/6/00016f769000", "mashu/6/6/00016f769000"),
        ("mashu/6/6/0001", "mashu/6/6/000100000000"),
        ("slither/xf/3/2/h17d", "slither/f/3/2/h17d"),
        ("slither/1/1/.", "slither/1/1/./"),
        ("slither/v:abc/3/2/h17d", "slither/v:abc/3/2/h17d"),
    ] {
        assert_eq!(canonical_url(input, &opts()).unwrap(), want, "{input}");
    }
}

#[test]
fn diagnostics_and_strict() {
    assert!(codes("slither/3/2/h17d").is_empty());
    assert_eq!(codes("slither/3/2/h17dzzz"), [DiagnosticCode::TrailingData]);
    assert_eq!(codes("mashu/6/6/0001"), [DiagnosticCode::TruncatedBody]);
    assert_eq!(codes("slither/xf/3/2/h17d"), [DiagnosticCode::UnknownFlag]);
    assert_eq!(codes("mashu/f/3/3"), [DiagnosticCode::UnknownFlag]);
    assert!(codes("slither/3/2/H").contains(&DiagnosticCode::UnexpectedChar));
    assert_eq!(codes("mashu/2/1/q"), [DiagnosticCode::TrailingData]);
    assert_eq!(
        codes("https://x/p?foo=1&slither/3/2/h17d"),
        [DiagnosticCode::UnknownParameter]
    );
    assert!(matches!(
        Board::from_url("slither/3/2/h17dzzz", &strict()),
        Err(Error::Strict(_))
    ));
    assert!(Board::from_url("slither/3/2/h17d", &strict()).is_ok());
}

#[test]
fn flags_without_body_are_ignored_like_pzprjs() {
    let b = Board::from_url("mashu/f/3/3", &opts()).unwrap();
    assert!(!b.rules()["loop_full"]);
    assert_eq!(b.to_url().unwrap(), "mashu/3/3/000");
}

#[test]
fn intentional_divergences() {
    for bad in [
        "slither/0/2/0",
        "slither/a/b/c",
        "slither/2",
        "slither/2000/2/0",
    ] {
        assert!(
            matches!(Board::from_url(bad, &opts()), Err(Error::InvalidSize(_))),
            "{bad}"
        );
    }
    assert!(matches!(
        Board::from_url("v:x/slither/3/2/h17d", &opts()),
        Err(Error::UnsupportedUrlType(_))
    ));
    assert!(matches!(
        Board::from_url("pzprv3/slither/1/1/0", &opts()),
        Err(Error::UnsupportedUrlType(_))
    ));
    assert!(matches!(
        Board::from_url("nurikabe/3/3/a", &opts()),
        Err(Error::UnsupportedPuzzle(_))
    ));
    assert!(matches!(
        Board::from_url("nosuchpuzzle/3/3/a", &opts()),
        Err(Error::UnknownPuzzle(_))
    ));
    assert!(matches!(
        Board::from_fileio("pzprv3.2/slither/1/1/. /0 /0 0 /0 /0 /", &opts()),
        Err(Error::UnsupportedFileVersion(_))
    ));
    assert!(matches!(
        Board::from_fileio("pzprv3/slither/1/1/x /0 0 /0 /0 /", &opts()),
        Err(Error::InvalidFileIo { .. })
    ));
}

#[test]
fn out_of_range_values() {
    let b = Board::from_fileio("pzprv3/mashu/1/2/3 . /0 /", &opts()).unwrap();
    assert_eq!(b.diagnostics()[0].code, DiagnosticCode::ValueOutOfRange);
    assert!(matches!(b.to_url(), Err(Error::ValueOutOfRange(_))));
    let b = Board::from_fileio("pzprv3/mashu/1/2/- 1 /0 /", &opts()).unwrap();
    assert_eq!(b.diagnostics()[0].code, DiagnosticCode::LossyUrl);
    assert_eq!(b.to_url().unwrap(), "mashu/2/1/3");
}

#[test]
fn fileio_metadata_and_trials_are_reported() {
    let text = "pzprv3\nmashu\n1\n1\n1 \nhistory:{\"x\":1}\n";
    let b = Board::from_fileio(text, &opts()).unwrap();
    assert_eq!(b.diagnostics()[0].code, DiagnosticCode::DroppedHistory);
    let text = "pzprv3\nmashu\n1\n1\n1 \n\nTrialData(1)\n1 \n\n";
    let b = Board::from_fileio(text, &opts()).unwrap();
    assert_eq!(b.diagnostics()[0].code, DiagnosticCode::DroppedTrial);
}

fn solved_slither() -> Board {
    Board::from_fileio("pzprv3.1/slither/1/2/1 - /0 2 /0 1 1 /-1 0 /0 0 /", &opts()).unwrap()
}

#[test]
fn trace_urls() {
    let b = solved_slither();
    let url = b.to_trace_url("https://pzpr-trace.net/p?").unwrap();
    assert!(url.ends_with("&slither/2/1/1./"), "{url}");
    let back = Board::from_trace_url(&url, &opts()).unwrap();
    assert_eq!(back, b);

    let with_trace = url.replace("?state=", "?trace=abc&state=");
    assert_eq!(
        Board::from_trace_url(&with_trace, &opts()),
        Err(Error::TraceNotSupported)
    );
    let ignored = Board::from_trace_url(
        &with_trace,
        &Options {
            ignore_trace: true,
            ..opts()
        },
    )
    .unwrap();
    assert_eq!(ignored.diagnostics()[0].code, DiagnosticCode::IgnoredTrace);

    let state = b.to_state().unwrap();
    let wrong_clue = format!("https://pzpr-trace.net/p?state={state}&slither/2/1/2./");
    assert!(matches!(
        Board::from_trace_url(&wrong_clue, &opts()),
        Err(Error::StateMismatch(_))
    ));
    let wrong_rule = format!("https://pzpr-trace.net/p?state={state}&slither/f/2/1/1./");
    assert!(matches!(
        Board::from_trace_url(&wrong_rule, &opts()),
        Err(Error::StateMismatch(_))
    ));

    let plain = Board::from_url(&url, &opts()).unwrap();
    assert!(!plain.has_snapshot());
    assert_eq!(plain.diagnostics()[0].code, DiagnosticCode::IgnoredState);
}

#[test]
fn state_is_strict_pzpr_trace_encoding() {
    let b = solved_slither();
    let state = b.to_state().unwrap();
    assert_eq!(Board::from_state(&state, &opts()).unwrap(), b);
    for bad in [
        format!("{state}="),
        format!("{state}+"),
        format!("{} {}", &state[..4], &state[4..]),
        b.to_fileio().unwrap(),
        "!!!".to_string(),
    ] {
        assert!(
            matches!(Board::from_state(&bad, &opts()), Err(Error::StateDecode(_))),
            "{bad}"
        );
    }
    let limited = Options {
        max_state_bytes: 10,
        ..opts()
    };
    assert!(matches!(
        Board::from_state(&state, &limited),
        Err(Error::ResourceLimit(_))
    ));
}

#[test]
fn parse_detects_input_form() {
    let b = solved_slither();
    let state = b.to_state().unwrap();
    let trace = b.to_trace_url("https://pzpr-trace.net/p?").unwrap();
    for input in [b.to_fileio().unwrap(), state, trace] {
        assert_eq!(Board::parse(&input, &opts()).unwrap(), b, "{input}");
    }
    let puzzle = Board::parse("https://puzz.link/p?slither/2/1/1./", &opts()).unwrap();
    assert!(!puzzle.has_snapshot());
    assert_eq!(puzzle.to_url().unwrap(), "slither/2/1/1./");
}

#[test]
fn snapshot_follows_transforms() {
    let b = solved_slither();
    let t = b.transform(Transform::Rot90);
    assert_eq!((t.cols(), t.rows()), (1, 2));
    assert!(t.has_snapshot());
    assert_eq!(t.transform(Transform::Rot270), b);
    assert_eq!(t.to_url().unwrap(), "slither/1/2/1./");
}

#[test]
fn builder() {
    let mut b = Board::new("masyu", 3, 1).unwrap();
    b.cell_mut(0, 0).qnum = 1;
    b.cell_mut(0, 2).qnum = 2;
    b.set_rule("loop_full", true).unwrap();
    assert_eq!(b.to_url().unwrap(), "mashu/f/3/1/b");
    assert!(matches!(
        b.set_rule("slither_full", true),
        Err(Error::UnknownRule(_))
    ));
}
