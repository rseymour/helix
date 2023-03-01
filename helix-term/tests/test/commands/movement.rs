use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn expand_shrink_selection() -> anyhow::Result<()> {
    let tests = vec![
        // single range
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
            "<A-o><A-o>",
            helpers::platform_line(indoc! {r##"
                #[Some(thing)|]#
            "##}),
        ),
        // multi range
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
                Some(#(other_thing|)#)
            "##}),
            "<A-o>",
            helpers::platform_line(indoc! {r##"
                Some#[(thing)|]#
                Some#((other_thing)|)#
            "##}),
        ),
        // multi range collision merges
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o>",
            helpers::platform_line(indoc! {r##"
                #[(
                    Some(thing),
                    Some(other_thing),
                )|]#
            "##}),
        ),
        // multi range collision merges, then shrinks back to original
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[Some(thing)|]#,
                    #(Some(other_thing)|)#,
                )
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    Some#[(thing)|]#,
                    Some#((other_thing)|)#,
                )
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o><A-i><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
        ),
        // shrink with no expansion history defaults to first child
        (
            helpers::platform_line(indoc! {r##"
                (
                    #[Some(thing)|]#,
                    Some(other_thing),
                )
            "##}),
            "<A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[Some|]#(thing),
                    Some(other_thing),
                )
            "##}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}
