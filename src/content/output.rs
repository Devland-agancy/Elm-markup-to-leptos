
    view! {
        <Section>
            <Paragraph>
                <span class="text">r#"asdasd"#</span>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">r#"Mind you, concerning this example, that "#</span>
                    <MathBlock>r#"$$ \Large \{\{\} \} \ne \{\} $$"#</MathBlock>
                </Indent>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"because a box containing an empty box is not the same thing as an empty box! Specifically, "#
                </span>
                <MathBlock>r#"$$ \Large \{ \{\} \} $$"#</MathBlock>
                <span class="text">
                    r#"answers “yes” to the question “do you contain "#<span class="nobreak">
                        <Math>r#"$\{\}$"#</Math>
                        r#"?”  "#
                    </span>r#" (a.k.a., “do you contain "#<span class="nobreak">
                        <Math>r#"$\phi$"#</Math>
                        r#"?”)  "#
                    </span>r#" whereas             "#
                </span>
                <MathBlock>r#"$$ \Large \{\} $$"#</MathBlock>
                <span class="text">
                    r#"answers “no” to the same question. (Indeed, while the empty set "#
                    <Span italic=true>r#"contains"#</Span>r#" nothing, it "#
                    <Span italic=true>r#"is"#</Span>r#" something.) Similarly,     "#
                </span>
                <MathBlock>r#"$$ \Large \{\{\{\}\} \} \ne \{\{\}\} $$"#</MathBlock>
                <span class="text">
                    r#"etc, etc: adding a new outer layer changes the whole set each time.                "#
                </span>
            </Paragraph>
        </Section>
    }
    