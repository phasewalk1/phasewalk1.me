pub(super) fn Sheet() -> String {
    return r#"
        .large-text {
            font-size: 3em;
        }
        .medium-text {
            font-size: 1.5em;
        }
        .list {
            font-size: 1.25em;
            flex-direction: column;
            display: flex;
        }
        .subhead {
            font-size: 1.3em;
        }
        .container {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            height: 100vh;
        }
    "#.to_string();
}
