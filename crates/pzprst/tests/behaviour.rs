//! Spec examples, diagnostics, intentional divergences, states and trace URLs.

use pzprst::{
    canonical_url, BankPiece, Board, DiagnosticCode, Error, Group, Options, PieceBank, Transform,
};

fn opts() -> Options {
    Options::default()
}

#[test]
fn piece_bank_roundtrip_and_state_identity() {
    let raw = BankPiece::from_serialized("2:110111").unwrap();
    assert_eq!(
        BankPiece::from_serialized(&raw.serialize().unwrap()).unwrap(),
        raw
    );
    let mut board = Board::from_url("statuepark/2/2/00/2/23bg/22u", &opts()).unwrap();
    assert!(!board.has_snapshot());
    board.bank.as_mut().unwrap().pieces[0].qcmp = 1;
    assert!(board.has_snapshot());
    let trace = board.to_trace_url("https://pzpr-trace.net/p?").unwrap();
    assert_eq!(Board::from_trace_url(&trace, &opts()).unwrap(), board);
    let mut changed = board.clone();
    changed.bank = Some(PieceBank::from_serialized(["23bg"]).unwrap());
    let mismatch = format!(
        "https://pzpr-trace.net/p?state={}&{}",
        board.to_state().unwrap(),
        changed.to_url().unwrap()
    );
    assert!(matches!(
        Board::from_trace_url(&mismatch, &opts()),
        Err(Error::StateMismatch(_))
    ));
    board.clear_snapshot();
    assert!(!board.has_snapshot());
    assert_eq!(board.bank.as_ref().unwrap().pieces[0].qcmp, 0);
    board.bank = None;
    assert!(matches!(board.to_url(), Err(Error::ValueOutOfRange(_))));
    assert!(matches!(board.to_fileio(), Err(Error::ValueOutOfRange(_))));
}

#[test]
fn bosanowa_crop_and_unrepresentable_clues_are_reported() {
    let mut board = Board::new("bosanowa", 5, 4).unwrap();
    assert!(!board.has_snapshot());
    for y in 0..board.rows() {
        for x in 0..board.cols() {
            board.cell_mut(y, x).ques = 7;
        }
    }
    board.cell_mut(1, 1).ques = 0;
    board.cell_mut(1, 2).ques = 0;
    board.cell_mut(1, 1).qnum = 2;
    assert_eq!(board.to_url().unwrap(), "bosanowa/2/1/02g");
    assert!(matches!(
        board.to_trace_url("https://pzpr-trace.net/p?"),
        Err(Error::StateMismatch(_))
    ));
    let restored = Board::from_fileio(&board.to_fileio().unwrap(), &opts()).unwrap();
    assert!(restored
        .diagnostics()
        .iter()
        .any(|d| d.code == DiagnosticCode::LossyUrl));
    assert_eq!((restored.cols(), restored.rows()), (5, 4));
    board.obj_mut(Group::Border, 2, 1).unwrap().qsub = 1;
    assert!(board.has_snapshot());
    board.clear_snapshot();
    assert!(!board.has_snapshot());
    let mut large = Board::new("bosanowa", 1, 1).unwrap();
    large.cell_mut(0, 0).qnum = 256;
    let reread = Board::from_fileio(&large.to_fileio().unwrap(), &opts()).unwrap();
    assert!(reread
        .diagnostics()
        .iter()
        .any(|d| d.code == DiagnosticCode::LossyUrl));
    assert!(matches!(
        large.to_trace_url("https://pzpr-trace.net/p?"),
        Err(Error::StateMismatch(_))
    ));
    let large_url = Board::from_url("bosanowa/1/1/0+100", &opts()).unwrap();
    assert!(large_url
        .diagnostics()
        .iter()
        .any(|d| d.code == DiagnosticCode::LossyUrl));
    assert!(Board::from_url(
        "bosanowa/1/1/0+100",
        &Options {
            strict: true,
            ..opts()
        }
    )
    .is_err());
    // Reference state matching normalizes question data through FileIO, where
    // a zero clue becomes empty; h is an unstored display setting.
    let zero = Board::from_url("bosanowa/h/1/1/00", &opts()).unwrap();
    let trace = zero.to_trace_url("https://pzpr-trace.net/p?").unwrap();
    let decoded = Board::from_trace_url(&trace, &opts()).unwrap();
    assert_eq!(decoded.cell(0, 0).qnum, -1);
    assert!(decoded.to_url().unwrap().starts_with("bosanowa/h/"));
    let mut empty = Board::new("bosanowa", 1, 1).unwrap();
    empty.cell_mut(0, 0).ques = 7;
    assert!(matches!(empty.to_url(), Err(Error::InvalidSize(_))));
}

#[test]
fn sudoku_box_geometry_and_file_header() {
    let b = Board::new("sudoku", 6, 6).unwrap();
    assert_eq!(b.obj(Group::Border, 6, 1).unwrap().ques, 1);
    assert_eq!(b.obj(Group::Border, 4, 1).unwrap().ques, 0);
    assert_eq!(b.obj(Group::Border, 1, 4).unwrap().ques, 1);
    assert!(b.obj(Group::Cross, 0, 0).is_some());
    assert!(b.to_fileio().unwrap().starts_with("pzprv3\nsudoku\n6\n"));

    // The reference writes only cols, even for a rectangular URL. Its FileIO
    // parser then constructs a square board from that single dimension.
    let rectangular = Board::from_url("sudoku/6/4/", &opts()).unwrap();
    let restored = Board::from_fileio(&rectangular.to_fileio().unwrap(), &opts()).unwrap();
    assert_eq!((restored.cols(), restored.rows()), (6, 6));
}

#[test]
fn tapa_multiple_clues_are_part_of_state_identity() {
    let a = Board::from_url("tapa/1/1/a8", &opts()).unwrap();
    let b = Board::from_url("tapa/1/1/a9", &opts()).unwrap();
    assert_ne!(a.cell(0, 0).qnums, b.cell(0, 0).qnums);
    let trace = format!(
        "https://pzpr-trace.net/p?state={}&{}",
        a.to_state().unwrap(),
        b.to_url().unwrap()
    );
    assert!(matches!(
        Board::from_trace_url(&trace, &opts()),
        Err(Error::StateMismatch(_))
    ));
}

#[test]
fn easyasabc_indicator_is_part_of_state_identity() {
    let a = Board::from_url("easyasabc/2/2/2/z", &opts()).unwrap();
    let b = Board::from_url("easyasabc/2/2/3/z", &opts()).unwrap();
    let trace = format!(
        "https://pzpr-trace.net/p?state={}&{}",
        a.to_state().unwrap(),
        b.to_url().unwrap()
    );
    assert!(matches!(
        Board::from_trace_url(&trace, &opts()),
        Err(Error::StateMismatch(_))
    ));
}

#[test]
fn starbattle_star_count_is_part_of_state_identity() {
    let a = Board::from_url("starbattle/2/2/1/00", &opts()).unwrap();
    let b = Board::from_url("starbattle/2/2/2/00", &opts()).unwrap();
    let trace = format!(
        "https://pzpr-trace.net/p?state={}&{}",
        a.to_state().unwrap(),
        b.to_url().unwrap()
    );
    assert!(matches!(
        Board::from_trace_url(&trace, &opts()),
        Err(Error::StateMismatch(_))
    ));
    let valid = a.to_trace_url("https://pzpr-trace.net/p?").unwrap();
    assert_eq!(Board::from_trace_url(&valid, &opts()).unwrap(), a);
}

#[test]
fn arrowflow_directional_submarks_survive_fileio_and_transforms() {
    let text = "pzprv3\narrowflow\n1\n1\n. \n+3 \n";
    let board = Board::from_fileio(text, &opts()).unwrap();
    assert_eq!(board.cell(0, 0).qsub, 3);
    assert_eq!(board.to_fileio().unwrap(), text);
    assert_eq!(board.transform(Transform::Rot90).to_fileio().unwrap(), text);
}

#[test]
fn bonsan_legacy_room_borders_select_heyabon() {
    assert_eq!(
        Board::from_url("bonsan/2/2/100", &opts()).unwrap().pid(),
        "heyabon"
    );
    assert_eq!(
        Board::from_url("bonsan/2/2/001", &opts()).unwrap().pid(),
        "bonsan"
    );
    assert!(Board::from_url("bonsan/2/2/001", &opts())
        .unwrap()
        .to_url()
        .unwrap()
        .starts_with("bonsan/c/"));
}

#[test]
fn railpool_trace_accepts_reconstructed_blocked_cell_boundaries() {
    let b = Board::from_url(
        "railpool/6/6/00k10n00zq02l0o0/po003jgk005100008g00",
        &opts(),
    )
    .unwrap();
    let state = b.to_state().unwrap();
    let trace = format!(
        "https://pzpr-trace.net/p?state={state}&{}",
        b.to_url().unwrap()
    );
    let restored = Board::from_trace_url(&trace, &opts()).unwrap();
    assert_eq!(restored.to_fileio().unwrap(), b.to_fileio().unwrap());
}

#[test]
fn segment_state_and_reference_transform_errors() {
    let mut b = Board::new("kouchoku", 1, 5).unwrap();
    b.segments.push(pzprst::Segment {
        start: (2, 2),
        end: (2, 4),
        origin: None,
    });
    assert!(b.has_snapshot());
    assert!(matches!(
        b.try_transform(Transform::Rot90),
        Err(Error::InvalidTransform(_))
    ));
    assert_eq!(
        b.transform(Transform::Rot90).transform(Transform::Rot270),
        b
    );
    assert_eq!(
        Board::from_state(&b.to_state().unwrap(), &opts()).unwrap(),
        b
    );
    b.clear_snapshot();
    assert!(!b.has_snapshot());
}

#[test]
fn icebarn_legacy_applets_and_icelom_variant() {
    assert_eq!(
        canonical_url(
            "http://indi.s58.xrea.com/icebarn/sa/?/2/2/0/////0/1",
            &opts()
        )
        .unwrap(),
        "icebarn/2/2/044/0/1"
    );
    assert_eq!(
        canonical_url("http://indi.s58.xrea.com/icebarn/sa/?c/2/2/0/1/0", &opts()).unwrap(),
        "icebarn/2/2/0123/2/0"
    );
    assert_eq!(
        Board::from_url("icelom/4/4/0", &opts()).unwrap().pid(),
        "icelom2"
    );
    assert_eq!(
        Board::from_url("icelom/a/4/4/0", &opts()).unwrap().pid(),
        "icelom"
    );
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
        Board::from_url("curvedata/3/3/a", &opts()),
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
fn pencil_marks_round_trip_and_clear() {
    let input = "pzprv3/hakoiri/1/1/1/0/./.[1,,2,]/";
    let mut board = Board::from_fileio(input, &opts()).unwrap();
    assert_eq!(board.cell(0, 0).snum, vec![1, -1, 2, -1]);
    assert!(board.has_snapshot());
    assert_eq!(
        Board::from_state(&board.to_state().unwrap(), &opts()).unwrap(),
        board
    );
    board.clear_snapshot();
    assert!(!board.has_snapshot());
    assert!(board.cell(0, 0).snum.is_empty());
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
