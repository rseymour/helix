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
                #[(
                    Some(thing),
                    Some(other_thing),
                )|]#
            "##}),
            "<A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[Some(thing)|]#,
                    Some(other_thing),
                )
            "##}),
        ),
        // any movement cancels selection history and falls back to first child
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )

            "##}),
            "<A-o><A-o><A-o>jkvkkk<A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[|Some(thing)]#,
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

#[tokio::test(flavor = "multi_thread")]
async fn expand_selection_around() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
            "<A-O><A-O>",
            helpers::platform_line(indoc! {r##"
                #[Some(|]#thing#()|)#
            "##}),
        ),
        // shrinking restores previous selection
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
            "<A-O><A-O><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
        ),
        // multi range collision merges expand as normal, except with the
        // original selection removed from the result
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-O><A-O><A-O>",
            helpers::platform_line(indoc! {r##"
                #[(
                    Some(|]#thing#(),
                    Some(|)#other_thing#(),
                )|)#
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-O><A-O><A-O><A-i><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}
