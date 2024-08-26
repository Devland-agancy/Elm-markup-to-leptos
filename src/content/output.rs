
    view! {
        <Section>
            <Paragraph>""</Paragraph>
            <span class="text">
                <Span bold=true>r#"Square Roots."#</Span>
                r#" You might remember that “minus times minus is plus” and that “plus times plus is plus”. (Why? The enemy of my enemy is my friend.) So any nonzero number multiplied by itself is positive. For example,                "#
            </span>
            <Paragraph>
                <MathBlock>r#"$$ (-2) \times (-2) = 4 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <Span align=Align::Center>r#"and"#</Span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ 2 \times 2 = 4 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"are both positive. But "#<Math>r#"$\sqrt{4}$"#</Math>
                    r#" is, by definition, the unique "#<Span italic=true>r#"nonnegative"#</Span>
                    r#" solution to "#<span class="nobreak">
                        <Math>r#"$x^2 = 4$"#</Math>
                        r#"."#
                    </span>r#" Hence, and whether you like it or not,"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$\sqrt{(-2)^2} = 2$$"#</MathBlock>
                <ImageRight
                    src="/images/svg_cloud_minus_two_squared.svg"
                    offset_y="-1rem"
                    offset_x="-3rem"
                    padding="40px"
                    use_squiggle_on_mobile=false
                >
                    <MathBlock>r#"$$ \sqrt{x^{2}} \rt{0.1} = \rt{0.1} x $$"#</MathBlock>
                </ImageRight>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"and, in particular, it is "#<Span italic=true>r#"not"#</Span>r#" true that"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ \sqrt{x^{2}} \rt{0.1} = \rt{0.1} x $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"for every real number "#<span class="nobreak">
                        <Math>r#"$x$"#</Math>
                        r#"."#
                    </span>r#" Instead we have"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ \sqrt{x^{2}} \rt{0.1} = \rt{0.1} |x| $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"for every real number "#<span class="nobreak">
                        <Math>r#"$x$"#</Math>
                        r#","#
                    </span>r#" where "#<Math>r#"$|x|$"#</Math>r#" denotes the absolute value of "#
                    <span class="nobreak">
                        <Math>r#"$x$"#</Math>
                        r#"."#
                    </span>
                </span>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">
                        r#"(Nb: If ever you want to indicate both solutions of the equation "#
                        <Math>r#"$x^2 = 4$"#</Math>r#" you can always use the notation "#
                        <span class="nobreak">
                            r#"“"#<Math>r#"$\pm \sqrt{4}$"#</Math>r#"”.  "#
                        </span>
                        r#" This is what happens, for example, in the maybe-well-known formula        "#
                    </span>
                </Indent>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ x = {-b \pm \sqrt{b^2 - 4ac} \over 2a} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"for the solutions to the quadratic equation "#<span class="nobreak">
                        <Math>r#"$ax^2 + bx + c = 0$"#</Math>
                        r#".)"#
                    </span>
                </span>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">r#"Now we can ponder, say,"#</span>
                </Indent>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ \sqrt{0.5} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"whose value is—by definition—the unique nonnegative solution to        "#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ x^2 = 0.5. $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"As beginners, there's nothing wrong with trying to solve this equation by trial and error. With "#
                    <span class="nobreak">
                        <Math>r#"$x = {1\over 4}$"#</Math>
                        r#","#
                    </span>r#" for example, we find"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ x^2 = {1\over 4}\times{1\over 4} = {1\over 16} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"so "#<Math>r#"$x = {1\over 4}$"#</Math>
                    r#" is not a solution of the equation, being apparently too small. Increasing "#
                    <Math>r#"$x$"#</Math>r#" to "#<span class="nobreak">
                        <Math>r#"$x = {1\over 2}$"#</Math>
                        r#","#
                    </span>r#" say, we find"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ x^2 = {1\over 2}\times{1\over 2} = {1\over 4} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"which is better, since "#<Math>r#"$1/4$"#</Math>r#" is closer to "#
                    <span class="nobreak">
                        <Math>r#"$1/2$"#</Math>
                        r#","#
                    </span>r#" but still too small. Increasing "#<Math>r#"$x$"#</Math>r#" by "#
                    <Math>r#"$1/4$"#</Math>r#" again, say, to "#<span class="nobreak">
                        <Math>r#"$x = {3\over 4}$"#</Math>
                        r#","#
                    </span>r#" we find"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ x^2 = {3\over 4}\times{3\over 4} = {9\over 16} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"which—surprise!—is actually pretty close to "#<span class="nobreak">
                        <Math>r#"$1/2$"#</Math>
                        r#","#
                    </span>r#" as "#<span class="nobreak">
                        <Math>r#"$1/2 = 8/16$"#</Math>
                        r#"."#
                    </span>r#" And since "#<span class="nobreak">
                        <Math>r#"$9/16 > 0.5$"#</Math>
                        r#","#
                    </span>r#" "#<Math>r#"$\sqrt{0.5}$"#</Math>r#" must be a little "#
                    <Span italic=true>r#"less"#</Span>r#" than "#<span class="nobreak">
                        <Math>r#"${3\over 4} = 0.75$"#</Math>
                        r#"."#
                    </span>r#"        "#
                </span>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">
                        r#"In last resort, and in reasonably good agreement with our observations, a calculator reveals that"#
                    </span>
                </Indent>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ \sqrt{0.5} = 0.7071067... $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"where the decimals trail off with no pattern. (This number is irrational.) Even so, the fact that "#
                    <Math>r#"$\sqrt{0.5}$"#</Math>r#" is "#<Span italic=true>r#"greater"#</Span>
                    r#" than "#<Math>r#"$0.5$"#</Math>r#" is often perceived as counterintuitive."#
                </span>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">
                        r#"You can think of it this way: multiplying a value by "#
                        <span class="nobreak">
                            <Math>r#"$0.7071$"#</Math>
                            r#","#
                        </span>r#" or approximately "#<span class="nobreak">
                            <Math>r#"$\sqrt{0.5}$"#</Math>
                            r#","#
                        </span>r#" is like taking "#<Math>r#"$70.71\%$"#</Math>
                        r#" of that value—for example, say,    "#
                    </span>
                </Indent>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ 605 \cdot 0.7071 = 427.7955 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"is "#<Math>r#"$70.71\%$"#</Math>r#" of "#<span class="nobreak">
                        <Math>r#"$605$"#</Math>
                        r#","#
                    </span>r#" and so on—so if we multiply "#<Span italic=true>r#"twice"#</Span>
                    r#" by "#<Math>r#"$0.7071$"#</Math>r#" we obtain “"#
                    <Math>r#"$70.71\%$"#</Math>r#" of "#<span class="nobreak">
                        <Math>r#"$70.71\%$"#</Math>
                        r#"”  "#
                    </span>r#" and it just so happens that “"#<Math>r#"$70.71\%$"#</Math>r#" of "#
                    <span class="nobreak">
                        <Math>r#"$70.71\%$"#</Math>
                        r#"”  "#
                    </span>r#" is close to "#<span class="nobreak">
                        <Math>r#"$50\%$"#</Math>
                        r#"."#
                    </span>r#"                    "#
                </span>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">
                        r#"The point is: if “"#<Math>r#"$X\%$"#</Math>r#" of "#
                        <span class="nobreak">
                            <Math>r#"$X\%$"#</Math>
                            r#"”  "#
                        </span>r#" equals "#<span class="nobreak">
                            <Math>r#"$50\%$"#</Math>
                            r#","#
                        </span>r#" then, of course, "#<span class="nobreak">
                            <Math>r#"$\rt{0.03}X > 50$"#</Math>
                            r#"—that  "#
                        </span>
                        r#" much seems logical—and, with a little thought, the same phenomenon explains why "#
                        <span class="nobreak">
                            <Math>r#"$\sqrt{0.5} > 0.5$"#</Math>
                            r#"."#
                        </span>r#"                "#
                    </span>
                </Indent>
            </Paragraph>
        </Section>
        <Section>
            <Paragraph>
                <span class="text">
                    <Span bold=true>r#"Fractions and Division."#</Span>
                    r#" An elementary fraction, or division, such as"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ {50 \over 2} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>""</Paragraph>
            <span class="text">r#"can be thought of in a few different ways:"#</span>
            <Paragraph>
                <List>
                    <Item>
                        <span class="text">
                            r#"Fifty halves (i.e., "#<span class="nobreak">
                                <Math>r#"$50 \times {1\over 2}$"#</Math>
                                r#")."#
                            </span>
                        </span>
                    </Item>
                    <Item>
                        <span class="text">
                            r#"The size obtained when something of size fifty is divided into two equal parts (answer: "#
                            <span class="nobreak">
                                <Math>r#"$25$"#</Math>
                                r#")."#
                            </span>
                        </span>
                    </Item>
                    <Item>
                        <span class="text">
                            r#"The number of times that "#<Math>r#"$2$"#</Math>r#" goes into "#
                            <Math>r#"$50$"#</Math>r#" (answer: "#<span class="nobreak">
                                <Math>r#"$25$"#</Math>
                                r#","#
                            </span>r#" because it takes twenty-five "#<span class="nobreak">
                                <Math>r#"$2$"#</Math>
                                r#"'s"#
                            </span>r#" to make up "#<span class="nobreak">
                                <Math>r#"$50$"#</Math>
                                r#")."#
                            </span>
                        </span>
                    </Item>
                    <Item>
                        <span class="text">
                            r#"our possible points of view are going to be more restricted. Thankfully, however, we can still characterize this fraction as the answer to the question “how many times does "#
                            <Math>r#"$0.01$"#</Math>r#" go into "#<span class="nobreak">
                                <Math>r#"$1$"#</Math>
                                r#"?”  "#
                            </span>r#" as in the third option above. And, indeed,        "#
                        </span>
                    </Item>
                </List>
            </Paragraph>
            <Paragraph>
                <Indent>
                    <span class="text">
                        r#"But "#<Math>r#"$50/2$"#</Math>
                        r#" is a ratio of integers, which makes things particularly nice! For a ratio of decimals, such as, say,"#
                    </span>
                </Indent>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ {1 \over 0.01} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <Item>""</Item>
                <Item>""</Item>
                <Item>""</Item>
                <Item>
                    <span class="text">
                        r#"our possible points of view are going to be more restricted. Thankfully, however, we can still characterize this fraction as the answer to the question “how many times does "#
                        <Math>r#"$0.01$"#</Math>r#" go into "#<span class="nobreak">
                            <Math>r#"$1$"#</Math>
                            r#"?”  "#
                        </span>r#" as in the third option above. And, indeed,        "#
                    </span>
                </Item>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ {1 \over 0.01} \,=\,100 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"because "#<Math>r#"$0.01$"#</Math>r#" goes "#<Math>r#"$100$"#</Math>
                    r#" times into "#<span class="nobreak">
                        <Math>r#"$1$"#</Math>
                        r#"."#
                    </span>r#" For that matter,"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>
                    r#"$$ { 1 \over 0.001} = 1000,\qquad{1 \over 0.0001} = 10000,\quad\,\,\,\,\textrm{(etc)} $$"#
                </MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"by the same reasoning, which explains why dividing by smaller and smaller numbers produces larger and larger results (and, by extension, why dividing by "#
                    <Math>r#"$0$"#</Math>r#" is undefined)."#
                </span>
            </Paragraph>
            <Paragraph>
                <Pause>""</Pause>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    <Span bold=true>r#"Note."#</Span>
                    r#" In general, the ratio of two decimal numbers can be turned into a ratio of integers by multiplying the ratio top and bottom by a suitable power of "#
                    <span class="nobreak">
                        <Math>r#"$10$"#</Math>
                        r#"."#
                    </span>
                    r#" E.g.:"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>
                    r#"$$ {1.42 \over 0.8} = {100 \cdot 1.42 \over 100 \cdot 0.8} = {142 \over 80} = {71 \over 40}. $$"#
                </MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"This example was chosen randomly, and, if you allow, we would like to see how large "#
                    <Math>r#"$71/40$"#</Math>r#" really is (one second!):"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>
                    r#"$$ \begin{align} {71 \over 40} \,&=\, {40 + 30 + 1 \over 40} \,=\, {40 \over 40} + {30 \over 40} + {1 \over 40}\\ \,&=\, 1 + {3 \over 4} + {1 \over 4}\!\cdot \!{1 \over 10}\up{1.5}\\ \,&=\, 1 + 0.75 + 0.025 = 1.775\up{1.5} \end{align} $$"#
                </MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"...so we find, among others, that "#<Math>r#"$71$"#</Math>r#" is exactly "#
                    <Math>r#"$77.5\%$"#</Math>r#" greater than "#<span class="nobreak">
                        <Math>r#"$40$"#</Math>
                        r#"."#
                    </span>r#" (Interesting, no?)"#
                </span>
            </Paragraph>
        </Section>
        <Section>
            <Indent>
                <Paragraph>
                    <span class="text">
                        <Span bold=true>r#"Distributivity."#</Span>
                        r#" As you might already know, a number that multiplies a sum can be brought “inside” the sum. For example,        "#
                    </span>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ 5(10 + 2) \,=\, 5\!\cdot\!10 \,+\, 5\!\cdot\!2 $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <span class="text">r#"(five times twelve equals fifty plus ten), or"#</span>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ a(b + c) = ab + ac $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <span class="text">
                        r#"more generally. This property is known as the "#
                        <Span italic=true>r#"distributivity of multiplication over addition"#</Span>
                        r#", or "#<Span italic=true>r#"distributivity"#</Span>r#" for short."#
                    </span>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"(We might finally clarify that "#
                            <span class="nobreak">r#"‘"#<Math>r#"$\cdot$"#</Math>r#"’  "#</span>
                            r#" means “times”, i.e., the same as "#
                            <span class="nobreak">
                                r#"‘"#<Math>r#"$\times$"#</Math>r#"’.  "#
                            </span>r#" Moreover, when we write                        "#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>""</Paragraph>
                <MathBlock>r#"$$ 5\!\cdot\!10 \,+\, 5\!\cdot\!2 $$"#</MathBlock>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"we really mean"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (5\!\cdot\!10) + (5\!\cdot\!2) $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"as opposed to something else, such as"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ ((5\!\cdot\!10) + 5)\!\cdot\! 2, $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"because multiplication takes precedence over addition, by default.)"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"A little more generally, one has such identities as"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(C + D) \,=\, aC + bC + aD + bD $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"that come from multiplying every term of the first parenthesis with every term of the second parenthesis. Indeed,"#
                        </span>
                        <ImageLeft src="/images/325.svg" line=1.5 offset_y="-10px">
                            <Indent>
                                <span class="text">
                                    r#"(The fact that "#<Math>r#"$13 \times 13$"#</Math>
                                    r#" is exactly one greater than "#
                                    <Math>r#"$12 \times 14$"#</Math>r#" is a bit curious indeed.)"#
                                </span>
                            </Indent>
                        </ImageLeft>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(C + D) = (a + b)C + (a + b)D $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"by one application of distributivity, while"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)C = aC + bC $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)D = aD + bD $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"by distributivity again."#</span>
                    </Indent>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Example>
                        <MathBlock>r#"$$ (a + b)^2 = a^2 + 2ab + b^2 $$"#</MathBlock>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    <Span bold=true>r#"Example 1. "#</Span>
                                    r#"One has"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <MathBlock>
                                    r#"$$ \begin{align} (10 + 2)(10 + 4) \,&=\, 10\!\cdot\!10 \,+\, 10\!\cdot\!4 \,+\, 2\!\cdot\!10 \,+\, 2\!\cdot\!4\\ \,&=\, 100 \,+\, 40 \,+\, 20 \,+\, 8\\ \,&=\, 168 \end{align} $$"#
                                </MathBlock>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"so "#<span class="nobreak">
                                        <Math>r#"$12 \times 14 = 168$"#</Math>
                                        r#"."#
                                    </span>
                                </span>
                            </Indent>
                        </Paragraph>
                    </Example>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Example>
                        <MathBlock>r#"$$ (a + b)(C + D) = aC + aD + bC + bD $$"#</MathBlock>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    <Span bold=true>r#"Example 2. "#</Span>
                                    r#"One has"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <MathBlock>
                                    r#"$$ \begin{align} (10 + 3)(10 + 3) \,&=\, 10\!\cdot\!10 \,+\, 10\!\cdot\!3 \,+\, 3\!\cdot\!10 \,+\, 3\!\cdot\!3\\ \,&=\, 100 \,+\, 30 \,+\, 30 \,+\, 9\\ \,&=\, 169 \end{align} $$"#
                                </MathBlock>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"so "#<span class="nobreak">
                                        <Math>r#"$13 \times 13 = 169$"#</Math>
                                        r#"."#
                                    </span>
                                </span>
                            </Indent>
                        </Paragraph>
                    </Example>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"(The fact that "#<Math>r#"$13 \times 13$"#</Math>
                            r#" is exactly one greater than "#<Math>r#"$12 \times 14$"#</Math>
                            r#" is a bit curious indeed.)"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"If we start from the afore-mentioned identity"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(C + D) \,=\, aC + bC + aD + bD $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"and set "#<span class="nobreak">
                                <Math>r#"$C = a$"#</Math>
                                r#","#
                            </span>r#" "#<span class="nobreak">
                                <Math>r#"$D = b$"#</Math>
                                r#","#
                            </span>r#" we find"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(a + b) \,=\, aa + ba + ab + bb $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"or, equivalently,"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)^2 = a^2 + 2ab + b^2 $$"#</MathBlock>
                    <Paragraph>
                        <Indent>""</Indent>
                    </Paragraph>
                    <Paragraph>
                        <Indent>""</Indent>
                    </Paragraph>
                    <Paragraph>
                        <Indent>""</Indent>
                    </Paragraph>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"since "#<span class="nobreak">
                                <Math>r#"$(a + b)(a + b) = (a + b)^2$"#</Math>
                                r#","#
                            </span>r#" "#<Math>r#"$aa = a^2$"#</Math>r#" and "#
                            <span class="nobreak">
                                <Math>r#"$bb = b^2$"#</Math>
                                r#"."#
                            </span>r#" (This is the "#
                            <Span italic=true>r#"binomial expansion of degree two"#</Span>
                            r#", but such terminology is not very important at this stage.)"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Example no_padding=true no_padding=true>
                        <Example>
                            <MathBlock>
                                r#"$$ \begin{align} \up{1} (10 + 3)^2 \,&=\, 10\!\cdot\!10 \,+\, 2\!\cdot\!3\!\cdot\!10 \,+\, 3\!\cdot\!3 \\ \up{1} \,&=\, 100 + 60 + 9 \\ \up{1} \,&=\, 169 \end{align} $$"#
                            </MathBlock>
                            <Paragraph>
                                <Indent>
                                    <Indent>
                                        <span class="text">
                                            r#"is a difference of squares, "#<Math>r#"$19$"#</Math>
                                            r#" can be factored. (On the other hand "#
                                            <Math>r#"$19$"#</Math>
                                            r#" is a prime number, but nevermind.)"#
                                        </span>
                                    </Indent>
                                </Indent>
                            </Paragraph>
                        </Example>
                        <Paragraph>
                            <span class="text">
                                <Span bold=true>r#"Example 3. "#</Span>
                                r#"By the last formula (or “binomial expansion of degree two”),        "#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} \up{1} (10 + 3)^2 \,&=\, 10\!\cdot\!10 \,+\, 2\!\cdot\!3\!\cdot\!10 \,+\, 3\!\cdot\!3 \\ \up{1} \,&=\, 100 + 60 + 9 \\ \up{1} \,&=\, 169 \end{align} $$"#
                            </MathBlock>
                            <Paragraph>
                                <Indent>
                                    <Indent>
                                        <Paragraph>
                                            <Paragraph>
                                                <Indent>
                                                    <Indent>""</Indent>
                                                </Indent>
                                            </Paragraph>
                                        </Paragraph>
                                    </Indent>
                                </Indent>
                            </Paragraph>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"which agrees with Example 2."#</span>
                        </Paragraph>
                    </Example>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"On the other hand, setting "#<span class="nobreak">
                                <Math>r#"$C = a$"#</Math>
                                r#","#
                            </span>r#" "#<Math>r#"$D = -b$"#</Math>r#" in"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(C + D) = aC + aD + bC + bD $$"#</MathBlock>
                    <Paragraph>
                        <Indent>""</Indent>
                    </Paragraph>
                    <Paragraph>
                        <Indent>""</Indent>
                    </Paragraph>
                    <Paragraph>
                        <Indent>""</Indent>
                    </Paragraph>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"gives"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(a + (-b)) = aa + a(-b) + ba + b(-b) $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"or, less pedantically,"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(a - b) = aa - ab + ba - bb $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"or"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (a + b)(a - b) = a^2 - b^2 $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"since "#<span class="nobreak">
                                <Math>r#"$- ab + ba = 0$"#</Math>
                                r#","#
                            </span>r#" "#<span class="nobreak">
                                <Math>r#"$aa = a^2$"#</Math>
                                r#","#
                            </span>r#" "#<span class="nobreak">
                                <Math>r#"$bb = b^2$"#</Math>
                                r#"."#
                            </span>r#" Note that"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ a^2 - b^2 $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"is"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <Span italic=true align=Align::Center>
                        r#"a difference of squares"#
                    </Span>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"whence "#
                            <Span italic=true>
                                r#"a difference of squares can always be factored"#
                            </Span>r#". (Factored as "#<span class="nobreak">
                                <Math>r#"$(a + b)(a - b)$"#</Math>
                                r#","#
                            </span>
                            r#" that is.) (PS: “Factored” means “written as a product”.)                "#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph no_padding=true no_padding=true>
                    <Example>
                        <Paragraph>
                            <Indent>
                                <Indent>
                                    <span class="text">
                                        r#"is a difference of squares, "#<Math>r#"$19$"#</Math>
                                        r#" can be factored. (On the other hand "#
                                        <Math>r#"$19$"#</Math>
                                        r#" is a prime number, but nevermind.)"#
                                    </span>
                                </Indent>
                            </Indent>
                        </Paragraph>
                    </Example>
                    <Paragraph>""</Paragraph>
                    <Paragraph>
                        <Paragraph>
                            <Indent>
                                <Indent>
                                    <Paragraph>
                                        <Paragraph>
                                            <Indent>
                                                <Indent>""</Indent>
                                            </Indent>
                                        </Paragraph>
                                    </Paragraph>
                                </Indent>
                            </Indent>
                        </Paragraph>
                    </Paragraph>
                    <Paragraph>""</Paragraph>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Example>
                        <Indent>
                            <Indent>""</Indent>
                        </Indent>
                    </Example>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"In relation to distributivity, we should also mention the simple but important fact that multiplying a difference by "#
                            <Math>r#"$-1$"#</Math>r#" "#<Span italic=true>r#"reverses"#</Span>
                            r#" the difference. That is,"#
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ (-1)(a - b) \,=\, b - a $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"or, for short,"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ -(a - b) \,=\, b - a $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"because, indeed,"#</span>
                    </Indent>
                </Paragraph>
                <Paragraph>
                    <MathBlock>
                        r#"$$ \begin{align} (-1)(a - b) \,&=\, (-1)(a + (-b)) \\ \,&=\, (-1)a + (-1)(-b) \\ \,&=\, -a + b \end{align} $$"#
                    </MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">r#"by distributivity (used in the second step)."#</span>
                    </Indent>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Example>
                        <Paragraph>
                            <span class="text">
                                <Span bold=true>r#"Example 6. "#</Span>
                                r#"We have "#
                                <span class="nobreak">
                                    <Math>r#"$-(10 - 3) = 3 - 10$"#</Math>
                                    r#"."#
                                </span>
                                r#" (Because "#
                                <span class="nobreak">
                                    <Math>r#"$-7 = -7$"#</Math>
                                    r#","#
                                </span>
                                r#" as it would be, haha.)"#
                            </span>
                        </Paragraph>
                    </Example>
                </Paragraph>
            </Indent>
        </Section>
        <Section divider=false>
            <Paragraph>
                <span class="text">
                    <Span bold=true>r#"Epilogue."#</Span>
                    r#" Do you remember the near miss between"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ 12\cdot 14 \,=\, 168 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">r#"and"#</span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ 13 \cdot 13 \,=\, 13^2 \,=\, 169 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">r#"...? Well if you observe, additionally, that"#</span>
            </Paragraph>
            <Paragraph>
                <MathBlock>
                    r#"$$ \begin{align} 11\,\cdot\,13 &= 12^2 - 1\\ 10\,\cdot\,12 &= 11^2 - 1\\ 9\,\cdot\,11 &= 10^2 - 1 \end{align} $$"#
                </MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"(etc) you might become suspicious of a pattern! But the mystery is rather thin: we have"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ (n - 1)(n + 1) \,=\, n^2 - 1 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>""</Paragraph>
            <span class="text">
                r#"for "#<Span italic=true>r#"every"#</Span>r#" real number "#<Math>r#"$n$"#</Math>
                r#" because of the formula"#
            </span>
            <Paragraph>
                <MathBlock>r#"$$ (a - b)(a + b) \,=\, a^2 - b^2 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">r#"for a difference of squares!"#</span>
            </Paragraph>
        </Section>
        <Section no_padding=true>
            <Paragraph>
                <span class="text">
                    <Span bold=true>r#"Vocabulary."#</Span>
                    r#" A pair of algebraic expressions of the form"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ a + b,\, a - b $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"is called a "#<Span italic=true>r#"conjugate pair"#</Span>r#". For example,"#
                </span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ n + 1,\, n - 1 $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">r#"is a conjugate pair, as is"#</span>
            </Paragraph>
            <Paragraph>
                <MathBlock>r#"$$ \sqrt{3} + \sqrt{2},\,\, \sqrt{3} - \sqrt{2} $$"#</MathBlock>
            </Paragraph>
            <Paragraph>
                <span class="text">
                    r#"and so on. (Generally speaking, conjugate pairs are good things to multiply together.)"#
                </span>
            </Paragraph>
        </Section>
        <Exercises labels=vec!["0", "1"]>
            <Exercise>
                <Paragraph>
                    <span class="text">
                        <Span bold=true>r#"Exercise 1. "#</Span>
                        r#"True or false (and, if possible, explain):"#
                    </span>
                </Paragraph>
                <Paragraph>
                    <Grid cols=3 sm_cols=2 sm_cutoff=520 center_on_overflow=true id=0>
                        <Span>
                            <span class="text">r#"a. "#<Math>r#"$ 0.9^2 < 0.9 $"#</Math></span>
                        </Span>
                        <Span>
                            <span class="text">
                                r#"b. "#<Math>r#"$ \sqrt{0.01} = 0.1 $"#</Math>
                            </span>
                        </Span>
                        <Span>
                            <span class="text">
                                r#"c. "#
                                <Math>
                                    r#"$ \sqrt[2]{\up{0.8}\sqrt[3]{2}} = \sqrt[3]{\up{0.8}\sqrt[2]{2}} $"#
                                </Math>
                            </span>
                        </Span>
                        <Span>
                            <span class="text">
                                r#"d. "#<Math>r#"${\sqrt{2} \over \up{0.55}2} = \sqrt{0.5}$"#</Math>
                            </span>
                        </Span>
                        <Span>
                            <span class="text">
                                r#"e. "#<Math>r#"$ {1 \over \sqrt{2}} = \sqrt{0.5} $"#</Math>
                            </span>
                        </Span>
                        <Span>
                            <span class="text">r#"f. "#<Math>r#"$ 2^{30} > 1000^3 $"#</Math></span>
                        </Span>
                        <Span>
                            <span class="text">
                                r#"g. "#<Math>r#"$ {1 \over 0.95} > 1.05 $"#</Math>
                            </span>
                        </Span>
                        <Span>
                            <span class="text">r#"h. "#<Math>r#"$ (-1)^{101} = -1 $"#</Math></span>
                        </Span>
                        <Span>
                            <span class="text">
                                r#"i. "#
                                <Math>
                                    r#"$ {100 \over \up{0.5}99} < {101 \over \up{0.5}100} $"#
                                </Math>
                            </span>
                        </Span>
                    </Grid>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Solution solution_number=0>
                        <Paragraph>
                            <span class="text">r#"Part by part:"#</span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">
                                    r#"for all "#<span class="nobreak">
                                        <Math>r#"$x \geq 0$"#</Math>
                                        r#","#
                                    </span>r#" "#<Math>r#"$y > 0$"#</Math>
                                    r#" (you need each root to be defined), so"#
                                </span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"a. (True) We have"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ 0.9^2 = {9 \over 10}\cdot{9 \over 10} = {81 \over 100} = 0.81 $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"and "#<span class="nobreak">
                                    <Math>r#"$0.81 < 0.9$"#</Math>
                                    r#"."#
                                </span>
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">
                                    <Span italic=true>r#"Note 1."#</Span>
                                    r#" One can also proceed by “direct verification”:        "#
                                </span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"b. (True) We have"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ 0.1^2 = {1 \over 10} \cdot {1 \over 10} = {1 \over 100} = 0.01, $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"and "#<Math>r#"$0.1$"#</Math>r#" is nonnegative, so "#
                                <span class="nobreak">
                                    <Math>r#"$\sqrt{0.01} = 0.1$"#</Math>
                                    r#"."#
                                </span>
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">
                                    r#"e. (True) Using the "#
                                    <span class="nobreak">
                                        r#"“"#
                                        <Math>
                                            r#"${\sqrt{x} \over \sqrt{y}} = \sqrt{\up{0.7}x \over y}$"#
                                        </Math>r#"”  "#
                                    </span>r#" identity:        "#
                                </span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"c. (True) In fact, "#
                                <Math>r#"$\sqrt[2]{\up{0.75}\sqrt[3]{2}}$"#</Math>r#" and "#
                                <Math>r#"$\sqrt[3]{\up{0.75}\sqrt[2]{2}}$"#</Math>
                                r#" are both equal to "#<span class="nobreak">
                                    <Math>r#"$\sqrt[6]{\up{0.6}2}$"#</Math>
                                    r#"."#
                                </span>r#" To convince yourself, note that"#
                            </span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\,\, (\sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.1})^6 \\ =&\,\, \up{1.3} \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}}\qquad\\ =&\,\, \up{1.3} (\gbk\sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.11}) \times (\gbk\sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.11}) \times (\gbk\sqrt[2]{\up{0.75}\sqrt[3]{2}} \times \sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.11}) \\ =& \,\, \up{1.3} (\sqrt[3]{\up{0.64}2}\rt{0.1}) \times (\sqrt[3]{\up{0.64}2}\rt{0.1}) \times (\sqrt[3]{\up{0.64}2}\rt{0.1})\\ =& \,\, \up{1.4} 2 \end{align} $$"#
                            </MathBlock>
                            <ImageLeft src="/images/17.svg" offset_y="0.8rem">
                                <MathBlock>
                                    r#"$$ \left({1 \over \sqrt{2}}\right)^{\!2} = {1 \over \sqrt{2}}\cdot{1 \over \sqrt{2}} = {1 \over \sqrt{2}\cdot\sqrt{2}} = {1 \over 2} = 0.5. $$"#
                                </MathBlock>
                            </ImageLeft>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"and"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\,\, (\sqrt[3]{\up{0.75}\sqrt[2]{2}}\rt{0.1})^6 \\ =&\,\, \up{1.3} \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}}\\ =& \,\, \up{1.3} (\gbk\sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}}\rt{0.11}) \times (\gbk\sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times \sqrt[3]{\up{0.75}\sqrt[2]{2}}\rt{0.11})\\ =&\,\, \up{1.3} \sqrt[2]{\up{0.65}2} \times \sqrt[2]{\up{0.65}2}\\ =&\,\, \up{1.4} 2 \end{align} $$"#
                            </MathBlock>
                            <ImageLeft src="/images/18.svg">
                                <span class="text">
                                    r#"(The point being: we already know that "#
                                    <Math>r#"${\sqrt{2} \over 2} = \sqrt{0.5}$"#</Math>
                                    r#" by part d.)"#
                                </span>
                            </ImageLeft>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"so "#<span class="nobreak">
                                    <Math>
                                        r#"$(\sqrt[2]{\up{0.76}\sqrt[3]{2}}\rt{0.1})^6 = (\sqrt[3]{\up{0.76}\sqrt[2]{2}}\rt{0.1})^6 = 2$"#
                                    </Math>
                                    r#"."#
                                </span>
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"Technically, however, a number "#<Math>r#"$x$"#</Math>
                                    r#" such that"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ x^6 = 2 $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"is not necessarily "#<span class="nobreak">
                                    <Math>r#"$\sqrt[6]{\up{0.6}2}$"#</Math>
                                    r#","#
                                </span>r#" because "#<Math>r#"$x = -\sqrt[6]{\up{0.6}2}$"#</Math>
                                r#" satisfies this equation as well!"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"The last step, therefore, is to note that "#
                                    <Math>r#"$\sqrt[2]{\up{0.76}\sqrt[3]{2}}$"#</Math>r#" and "#
                                    <Math>r#"$\sqrt[3]{\up{0.76}\sqrt[2]{2}}$"#</Math>
                                    r#" are both "#<Span italic=true>r#"nonnegative"#</Span>
                                    r#" numbers (taken as obvious), and which implies that they are the "#
                                    <Span italic=true>r#"unique nonnegative"#</Span>
                                    r#" solution to "#<span class="nobreak">
                                        <Math>r#"$x^6 = 2$"#</Math>
                                        r#"."#
                                    </span>
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <Pause>
                                    <span class="text">
                                        <Span italic=true>r#"Note 3."#</Span>
                                        r#" More generally, even though"#
                                    </span>
                                </Pause>
                            </Pause>
                        </Paragraph>
                        <Paragraph>""</Paragraph>
                        <span class="text">r#"d. (True) In general,"#</span>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {\sqrt{x} \over \sqrt{y}} = \sqrt{\up{0.7}x \over y} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"for all "#<span class="nobreak">
                                    <Math>r#"$x \geq 0$"#</Math>
                                    r#","#
                                </span>r#" "#<Math>r#"$y > 0$"#</Math>
                                r#" (you need each root to be defined), so"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {\sqrt{2} \over 2} = {\sqrt{2} \over \sqrt{4}} = \sqrt{\up{0.8}2 \over 4} = \sqrt{0.5} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"...ta-daa!"#</span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <MathBlock>
                                    r#"$$ 2^{50}\fw\te{mm} = (2^{10})^5\fw\te{mm} \approx (10^3)^5\fw\te{mm} = 10^{15}\fw\te{mm}. $$"#
                                </MathBlock>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                <Span italic=true>r#"Note 1."#</Span>
                                r#" One can also proceed by “direct verification”:        "#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \left({\sqrt{2} \over 2}\right)^{\!2} = {\sqrt{2} \over 2}\cdot{\sqrt{2} \over 2} = {\sqrt{2}\cdot\sqrt{2} \over 4} = {2 \over 4} = 0.5. $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"(This, together with the fact that "#
                                <Math>r#"${\sqrt{2} \over 2}$"#</Math>
                                r#" is not negative, establishes that "#<span class="nobreak">
                                    <Math>r#"${\sqrt{2} \over 2} = \sqrt{0.5}$"#</Math>
                                    r#".)"#
                                </span>
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <MathBlock>r#"$$ 10^{9}\fw\te{km} $$"#</MathBlock>
                                <ImageRight
                                    src="/images/104.svg"
                                    offset_x="-4rem"
                                    offset_y="-1rem"
                                    children_y="48%"
                                    children_x="39%"
                                    use_squiggle_on_mobile=false
                                >
                                    <span class="text">
                                        <Math>
                                            r#"$10^{15}\fw\te{mm} = 10^{15}\fw(10^{-6}\fw\te{km}) = \dots$"#
                                        </Math>
                                    </span>
                                    <Pause>
                                        <span class="text">
                                            r#"of the sum? (By the way, this "#<span class="nobreak">
                                                <Math>r#"$n$"#</Math>
                                                r#"-th"#
                                            </span>r#" term is the difference "#
                                            <Math>r#"${1 \over 99} - {1 \over 100}$"#</Math>r#" for "#
                                            <span class="nobreak">
                                                <Math>r#"$n = 99$"#</Math>
                                                r#","#
                                            </span>
                                            r#" which is how we came to be reminded of this infinite sum in the first place.) Well..."#
                                        </span>
                                    </Pause>
                                </ImageRight>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"e. (True) Using the "#
                                <span class="nobreak">
                                    r#"“"#
                                    <Math>
                                        r#"${\sqrt{x} \over \sqrt{y}} = \sqrt{\up{0.7}x \over y}$"#
                                    </Math>r#"”  "#
                                </span>r#" identity:        "#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over \sqrt{2}} = {\sqrt{1} \over \sqrt{2}} = \sqrt{\up{0.8}1 \over 2} = \sqrt{0.5}. $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"Or by direct verification:"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \left({1 \over \sqrt{2}}\right)^{\!2} = {1 \over \sqrt{2}}\cdot{1 \over \sqrt{2}} = {1 \over \sqrt{2}\cdot\sqrt{2}} = {1 \over 2} = 0.5. $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"(And "#<Math>r#"$1 \over \sqrt{2}$"#</Math>
                                r#" is nonnegative.) Or by reducing to part d:"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over \sqrt{2}} = {\sqrt{2} \over \sqrt{2} \cdot \sqrt{2}} = {\sqrt{2} \over 2}. $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"(The point being: we already know that "#
                                <Math>r#"${\sqrt{2} \over 2} = \sqrt{0.5}$"#</Math>r#" by part d.)"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <MathBlock>
                                    r#"$$ \begin{align} & {1 \over 0.95} > 1.05\\ \iff & 1 > 1.05 \cdot 0.95\up{1.4}\\ \iff & 1 > (1 + 0.05)(1 - 0.05)\up{1.4}\\ \iff & 1 > 1 - 0.05^2\up{1.4} \end{align} $$"#
                                </MathBlock>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"f. (True) We have"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ 2^{30} = 2^{10} \times 2^{10} \times 2^{10} = (2^{10})^3 $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"and"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ (2^{10})^3 = (1024)^3 > 1000^3. $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">
                                    <Span italic=true>r#"Note 3."#</Span>
                                    r#" More generally, even though"#
                                </span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    <Span italic=true>r#"Note 2."#</Span>
                                    r#" The first ten or so powers of "#
                                    <Math>r#"$2$"#</Math>
                                    r#" are worth knowing by heart (here's "#
                                    <Span italic=true>r#"eleven"#</Span>
                                    r#" powers, mind you):"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{array}{c|c} \,\,\,\,n\,\,\,\, & 2^n\dn{0.3} \\ \hline 0 & 1 \up{1.1}\\ 1 & 2 \\ 2 & 4 \\ 3 & 8 \\ 4 & 16 \\ 5 & 32 \\ 6 & 64 \\ 7 & 128 \\ 8 & 256 \\ 9 & 512 \\ 10 & 1024 \end{array} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"Among which, the fact that"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ 2^{10} \approx 10^3 $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"can be particularly useful to know! For example, if a 1-millimeter-thick napkin is folded "#
                                <Math>r#"$50$"#</Math>
                                r#" times over, doubling the width each time, one obtains something of thickness"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ 2^{50}\fw\te{mm} = (2^{10})^5\fw\te{mm} \approx (10^3)^5\fw\te{mm} = 10^{15}\fw\te{mm}. $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"As"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ 1\fw\te{mm} = 10^{-6}\fw\te{km} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"this is"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ 10^{9}\fw\te{km} $$"#</MathBlock>
                            <ImageRight
                                src="/images/104.svg"
                                offset_x="-4rem"
                                offset_y="-1rem"
                                children_y="48%"
                                children_x="39%"
                                use_squiggle_on_mobile=false
                            >
                                <span class="text">
                                    <Math>
                                        r#"$10^{15}\fw\te{mm} = 10^{15}\fw(10^{-6}\fw\te{km}) = \dots$"#
                                    </Math>
                                </span>
                                <Pause>
                                    <span class="text">
                                        r#"of the sum? (By the way, this "#<span class="nobreak">
                                            <Math>r#"$n$"#</Math>
                                            r#"-th"#
                                        </span>r#" term is the difference "#
                                        <Math>r#"${1 \over 99} - {1 \over 100}$"#</Math>r#" for "#
                                        <span class="nobreak">
                                            <Math>r#"$n = 99$"#</Math>
                                            r#","#
                                        </span>
                                        r#" which is how we came to be reminded of this infinite sum in the first place.) Well..."#
                                    </span>
                                </Pause>
                            </ImageRight>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"or "#<Span italic=true>r#"one billion"#</Span>
                                r#" kilometers. By comparison, the distance from the Earth to the Sun is a mere "#
                                <Math>r#"$150$"#</Math>
                                r#" million kilometers. (The point being: that we could go from the relatively mysterious"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ \te{“}2^{50}\fw\te{mm}\te{”} $$"#</MathBlock>
                            r#"    "#
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"to the relatively less mysterious"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ \te{“}\fw10^{15}\te{mm}\te{”} $$"#</MathBlock>
                            r#"    "#
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"by the approximation "#<span class="nobreak">
                                    <Math>r#"$2^{10} \approx 10^3$"#</Math>
                                    r#".)"#
                                </span>
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">r#"and"#</span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"g. (True) As an inequality can be multiplied on both sides by a positive number while preserving the inequality, one has"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} & {1 \over 0.95} > 1.05\\ \iff & 1 > 1.05 \cdot 0.95\up{1.4}\\ \iff & 1 > (1 + 0.05)(1 - 0.05)\up{1.4}\\ \iff & 1 > 1 - 0.05^2\up{1.4} \end{align} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"(using the fact that "#<span class="nobreak">
                                    <Math>r#"$(1+x)(1-x) = 1-x^2$"#</Math>
                                    r#","#
                                </span>r#" of"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \te{“}\,(a+b)(a-b) = a^2-b^2\,\te{”} $$"#
                            </MathBlock>
                            r#"    "#
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"fame), and since the "#<Span italic=true>r#"last"#</Span>
                                r#" inequality is true, the "#<Span italic=true>r#"first"#</Span>
                                r#" inequality is true! (Recall that "#
                                <span class="nobreak">
                                    r#"“"#<Math>r#"$\!\iff\!$"#</Math>r#"”  "#
                                </span>r#" means “if and only if”.)                "#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">
                                    r#"is interesting in its own right, being connected to a famous infinite sum. To visualize this sum, picture a hare poised at "#
                                    <Math>r#"$x = 0$"#</Math>
                                    r#" on the number line. This hare runs forward by one unit and backwards by half a unit, stopping at the number"#
                                </span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                <Span italic=true>r#"Note 3."#</Span>
                                r#" More generally, even though"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 1 - \epsilon} > 1 + \epsilon $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"for any small "#<span class="nobreak">
                                    <Math>r#"$\epsilon > 0$"#</Math>
                                    r#","#
                                </span>r#" the number "#<Math>r#"$1 + \epsilon$"#</Math>
                                r#" remains a good approximation to "#<span class="nobreak">
                                    <Math>r#"${1 \over 1 - \epsilon}$"#</Math>
                                    r#"."#
                                </span>r#" For example,"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 0.99} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"is a good approximation to"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 0.99} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"while"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 0.999} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"is a good approximation to"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 0.999}, $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"etc."#</span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <span class="text">
                                    r#"of the sum? (By the way, this "#<span class="nobreak">
                                        <Math>r#"$n$"#</Math>
                                        r#"-th"#
                                    </span>r#" term is the difference "#
                                    <Math>r#"${1 \over 99} - {1 \over 100}$"#</Math>r#" for "#
                                    <span class="nobreak">
                                        <Math>r#"$n = 99$"#</Math>
                                        r#","#
                                    </span>
                                    r#" which is how we came to be reminded of this infinite sum in the first place.) Well..."#
                                </span>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"h. (True) Here are the first few powers of "#
                                <Math>r#"$-1$"#</Math>
                                r#" (note how each additional multiplication by "#
                                <Math>r#"$-1$"#</Math>
                                r#" simply changes the sign of the previous result):"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Grid cols=3 place_items="end" id=1>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)^1 = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1) = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ -1 $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)^2 = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)\times (-1) = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ 1 $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)^3 = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)\times(-1)\times (-1) = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ -1 $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)^4 = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)\times(-1)\times(-1)\times(-1) = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ 1 $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ (-1)^5 = $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>
                                            r#"$ \,\,\,(-1)\times(-1)\times(-1)\times(-1)\times(-1) = $"#
                                        </Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">
                                        <Math>r#"$ -1 $"#</Math>
                                    </span>
                                </Span>
                                <Span>
                                    <span class="text">r#"...it's that much. (For example,"#</span>
                                </Span>
                            </Grid>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"(Etc.) Obviously, even powers of "#<Math>r#"$(-1)$"#</Math>
                                    r#" are equal to "#<span class="nobreak">
                                        <Math>r#"$1$"#</Math>
                                        r#","#
                                    </span>r#" while odd powers of "#<Math>r#"$(-1)$"#</Math>
                                    r#" are equal to "#<span class="nobreak">
                                        <Math>r#"$-1$"#</Math>
                                        r#"."#
                                    </span>r#" As "#<Math>r#"$101$"#</Math>r#" is odd, "#
                                    <Math>r#"$(-1)^{101}$"#</Math>r#" is "#<span class="nobreak">
                                        <Math>r#"$-1$"#</Math>
                                        r#"."#
                                    </span>
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <MathBlock>r#"$$ {1 \over 100} - {1 \over 101} $$"#</MathBlock>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"i. (False) We have"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {100 \over 99} = {99 + 1 \over 99} = 1 + {1 \over 99} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"and"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {101 \over 100} = {100 + 1 \over 100} = 1 + {1 \over 100} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"so the smaller of the two fractions is "#<span class="nobreak">
                                    <Math>r#"${101 \over 100}$"#</Math>
                                    r#","#
                                </span>r#" since "#<span class="nobreak">
                                    <Math>r#"${1 \over 100} < {1 \over 99}$"#</Math>
                                    r#"."#
                                </span>
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>
                                <Indent>""</Indent>
                            </Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                <Span italic=true>r#"Note 4."#</Span>
                                r#" The difference"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 99} - {1 \over 100} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"is interesting in its own right, being connected to a famous infinite sum. To visualize this sum, picture a hare poised at "#
                                <Math>r#"$x = 0$"#</Math>
                                r#" on the number line. This hare runs forward by one unit and backwards by half a unit, stopping at the number"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ 1 - {1\over 2} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"by virtue of this back-and-forth movement. The hare then proceeds to run forward by "#
                                <Span italic=true>r#"half"#</Span>r#" a unit and back by a "#
                                <Span italic=true>r#"third"#</Span>r#" of a unit, stopping at"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\, \left(1 - {1 \over 2}\right) \\ + \,&\, \left({1 \over 2} - {1 \over 3}\right)_{\color{white} a_{a_a}\!\!\!\!\!\!\!\!\!\!} \\ \hline = \,&\, \left(1 - {1 \over 3}\right)^{\color{white} a^{a^a}} \end{align} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"for another break. Keeping with this pattern, the hare then stops at"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\, \left(1 - {1 \over 2}\right)\\ + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\ + \,&\, \left({1 \over 3} - {1 \over 4}\right)_{\color{white} a_{a_a}\!\!\!\!\!\!\!\!\!\!}\\ \hline = \,&\, \left(1 - {1 \over 4}\right)^{\color{white} a^{a^a}} \end{align} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"and then at"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\, \left(1 - {1 \over 2}\right)\\ + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\ + \,&\, \left({1 \over 3} - {1 \over 4}\right)\\ + \,&\, \left({1 \over 4} - {1 \over 5}\right)_{\color{white} a_{a_a}\!\!\!\!\!\!\!\!\!\!}\\ \hline = \,&\, \left(1 - {1 \over 5}\right)^{\color{white} a^{a^a}} \end{align} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"and so on. Clearly, the successive positions at which the hare stops are approaching the number "#
                                <Math>r#"$1$"#</Math>
                                r#" from the left, pointing to the fact that the "#
                                <Span italic=true>r#"infinite"#</Span>r#" sum"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\, \left(1 - {1 \over 2}\right)\\ + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\ + \,&\, \left({1 \over 3} - {1 \over 4}\right)\\ + \,&\, \left({1 \over 4} - {1 \over 5}\right)\\ + \,&\, \left({1 \over 5} - {1 \over 6}\right)\\ + \,&\, \left({1 \over 6} - {1 \over 7}\right)\\ + \,&\, \,\,\,\,\,\,\,\,\dots\up{1.3} \end{align} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"is “equal” (in some sense) to "#<span class="nobreak">
                                    <Math>r#"$1$"#</Math>
                                    r#"."#
                                </span>r#" But how much, exactly, is the "#<span class="nobreak">
                                    <Math>r#"$n$"#</Math>
                                    r#"-th"#
                                </span>r#" term        "#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over n} - {1 \over n+1} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"of the sum? (By the way, this "#<span class="nobreak">
                                    <Math>r#"$n$"#</Math>
                                    r#"-th"#
                                </span>r#" term is the difference "#
                                <Math>r#"${1 \over 99} - {1 \over 100}$"#</Math>r#" for "#
                                <span class="nobreak">
                                    <Math>r#"$n = 99$"#</Math>
                                    r#","#
                                </span>
                                r#" which is how we came to be reminded of this infinite sum in the first place.) Well..."#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} {1 \over n} - {1 \over n+1} &= {1 \over n}\cdot{n+1 \over n+1}\, - \, {1 \over n+1}\cdot{n \over n}\up{1.5}\\ &= {n+1 \over n(n+1)} - {n \over n(n+1)}\up{1.5}\\ &= {1 \over n(n+1)}\up{1.5} \end{align} $$"#
                            </MathBlock>
                            <ImageRight
                                src="/images/24.svg"
                                offset_y="3.5rem"
                                y="12%"
                                children_x="30%"
                                children_y="30%"
                            >
                                <MathBlock>
                                    r#"$$ \begin{align} {1 \over 99} - {1 \over 100} &= {1 \over 99}\cdot{100 \over 100}\, - \,{1 \over 100}\cdot{99 \over 99}\up{1.5}\\ &= {100 \over 99\cdot 100} - {99 \over 99\cdot 100}\up{1.5}\\ &= {1 \over 99\cdot 100}\up{1.5} \end{align} $$"#
                                </MathBlock>
                            </ImageRight>
                        </Paragraph>
                        <Paragraph>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>""</Span>
                            <Span>
                                <span class="text">r#"...it's that much. (For example,"#</span>
                            </Span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over 1} - {1 \over 2} = {1 \over 1 \cdot 2} = {1 \over 2} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"and"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over 2} - {1 \over 3} = {1 \over 2 \cdot 3} = {1 \over 6} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"and so on.) So the infinite sum"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} &\, \left(1 - {1 \over 2}\right)\\ + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\ + \,&\, \left({1 \over 3} - {1 \over 4}\right)\\ + \,&\, \left({1 \over 4} - {1 \over 5}\right)\\ + \,&\, \left({1 \over 5} - {1 \over 6}\right)\\ + \,&\, \left({1 \over 6} - {1 \over 7}\right)\\ + \,&\, \,\,\,\,\,\,\,\,\dots\up{1.3}\dn{1}\\ \hline = \,&\, 1\up{1.5} \end{align} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"can also be written"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over 1 \cdot 2} + {1 \over 2 \cdot 3} + {1 \over 3 \cdot 4} + {1 \over 4 \cdot 5} + {1 \over 5 \cdot 6} + \dots \,=\, 1 $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">r#"(or"#</span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over 2} + {1 \over 6} + {1 \over 12} + {1 \over 20} + {1 \over 30} + \dots \,=\, 1 $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"equivalently) which is not obvious at first glance, and kind of interesting!"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <Pause>""</Pause>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                <Span italic=true>r#"Note 5."#</Span>
                                r#" The fact that"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over n} - {1 \over n+1} = {1 \over n(n+1)} $$"#
                            </MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"means, in particular, that "#
                                <Math>r#"${1 \over n} - {1 \over n+1}$"#</Math>r#" is roughly "#
                                <Math>r#"${1 \over n^2}$"#</Math>r#" for large "#
                                <span class="nobreak">
                                    <Math>r#"$n$"#</Math>
                                    r#","#
                                </span>r#" which is sometimes handy to know. For example,"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 10} - {1 \over 11} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"is approximately "#<span class="nobreak">
                                    <Math>r#"$1/10^2 = 0.01$"#</Math>
                                    r#","#
                                </span>r#" while"#
                            </span>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 100} - {1 \over 101} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <span class="text">
                                r#"is approximately "#<span class="nobreak">
                                    <Math>r#"$1/100^2 = 0.01^2 = 0.0001$"#</Math>
                                    r#","#
                                </span>r#" etc."#
                            </span>
                        </Paragraph>
                    </Solution>
                </Paragraph>
            </Exercise>
            <Exercise>
                <Paragraph>
                    <span class="text">
                        <Span bold=true>r#"Exercise 2. "#</Span>
                        r#"In the solution to Exercise 1, we observed how the difference"#
                    </span>
                </Paragraph>
                <Paragraph>
                    <MathBlock>r#"$$ {1 \over n} - {1 \over n+1} $$"#</MathBlock>
                </Paragraph>
                <Paragraph>
                    <Indent>
                        <span class="text">
                            r#"is "#<Span italic=true>r#"roughly"#</Span>r#" "#
                            <Math>r#"$1/n^2$"#</Math>r#" for large "#<span class="nobreak">
                                <Math>r#"$n$"#</Math>
                                r#"."#
                            </span>
                            r#" But how far off is this exactly? (I.e., what is the difference between "#
                            <Math>r#"${1 \over n} - {1 \over n+1}$"#</Math>r#" and "#
                            <span class="nobreak">
                                <Math>r#"${1 \over n^2}$"#</Math>
                                r#"?)"#
                            </span>r#" And "#<Span italic=true>r#"roughly"#</Span>
                            r#" how much is this far-offness, for large "#<span class="nobreak">
                                <Math>r#"$n$"#</Math>
                                r#"?"#
                            </span>
                        </span>
                    </Indent>
                </Paragraph>
                <Paragraph no_padding=true>
                    <Solution solution_number=1>
                        <Paragraph>
                            <Indent>
                                <span class="text">r#"Since"#</span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ {1 \over n} - {1 \over n+1} = {1 \over n(n+1)} $$"#
                            </MathBlock>
                            <ImageRight
                                offset_x="-6rem"
                                offset_y="1.2rem"
                                src="/images/25.svg"
                                children_y="25%"
                                children_x="37%"
                                use_squiggle_on_mobile=false
                            >
                                <MathBlock>r#"$$ {1 \over 99\cdot 100} $$"#</MathBlock>
                            </ImageRight>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"(as previously computed) is smaller than"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over n^2} $$"#</MathBlock>
                            <ImageRight
                                offset_x="-6rem"
                                offset_y="1.2rem"
                                src="/images/26.svg"
                                children_y="26%"
                                children_x="37%"
                                use_squiggle_on_mobile=false
                            >
                                <MathBlock>r#"$$ {1 \over 99^2} $$"#</MathBlock>
                            </ImageRight>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">r#"we will compute the difference"#</span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over n^2} - {1 \over n(n+1)} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"as opposed to the “other difference    "#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over n(n+1)} - {1 \over n^2} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"to avoid a minus sign in the result. (Computing the second difference and having a minus sign does not constitute a mistake, however.) Having said this, the difference is:"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>
                                r#"$$ \begin{align} {1 \over n^2} - {1 \over n(n+1)} &= {1 \over n^2} \cdot {n+1 \over n+1} - {1 \over n(n+1)} \cdot {n \over n}\\ &= {n+1 \over n^2(n+1)} - {n \over n^2(n+1)}\up{1.5}\\ &= {1 \over n^2(n+1)}\up{1.5} \end{align} $$"#
                            </MathBlock>
                            <ImageRight
                                src="/images/27.svg"
                                offset_y="2rem"
                                y="12%"
                                children_y="31%"
                                children_x="23%"
                                clickable_on_desktop=true
                            >
                                <MathBlock>
                                    r#"$$ \begin{align} {1 \over 99^2} - {1 \over 99\cdot 100} &= {1 \over 99^2}\cdot{100 \over 100}\, - \, {1 \over 99\cdot 100}\cdot{99 \over 99}\\ &= {100 \over 99^2 \cdot 100} - {99 \over 99^2 \cdot 100}\up{1.5}\\ &= {1 \over 99^2\cdot 100}\up{1.5} \end{align} $$"#
                                </MathBlock>
                            </ImageRight>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"which, for large "#<span class="nobreak">
                                        <Math>r#"$n$"#</Math>
                                        r#","#
                                    </span>r#" is roughly"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over n^3} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"since "#<Math>r#"$n^2(n+1) \approx n^3$"#</Math>
                                    r#" for large "#<span class="nobreak">
                                        <Math>r#"$n$"#</Math>
                                        r#"."#
                                    </span>
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <Pause>""</Pause>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    <Span italic=true>r#"Example 1."#</Span>
                                    r#" Above, we estimated"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 10} - {1 \over 11} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"to be roughly "#<span class="nobreak">
                                        <Math>r#"$1/100 = 0.01$"#</Math>
                                        r#","#
                                    </span>r#" but "#<Math>r#"$1/100$"#</Math>
                                    r#" is bigger than the actual value of "#
                                    <Math>r#"${1 \over 10\cdot 11} = {1 \over 110}$"#</Math>r#" by"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 10^2\cdot 11} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"or close to "#<span class="nobreak">
                                        <Math>r#"$1/10^3 = 0.001$"#</Math>
                                        r#"."#
                                    </span>r#" (So"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 10} - {1 \over 11} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"is about "#<span class="nobreak">
                                        <Math>r#"$0.01$"#</Math>
                                        r#","#
                                    </span>r#" while making an error of about "#
                                    <span class="nobreak">
                                        <Math>r#"$0.001$"#</Math>
                                        r#".)"#
                                    </span>r#" (In fact,"#
                                </span>
                            </Indent>
                        </Paragraph>
                        <Paragraph>
                            <MathBlock>r#"$$ {1 \over 10^2\cdot 11} $$"#</MathBlock>
                        </Paragraph>
                        <Paragraph>
                            <Indent>
                                <span class="text">
                                    r#"is "#<Span italic=true>r#"less"#</Span>r#" than "#
                                    <span class="nobreak">
                                        <Math>r#"$1/10^3$"#</Math>
                                        r#","#
                                    </span>r#" so the error is "#<Span italic=true>r#"less"#</Span>
                                    r#" than "#<span class="nobreak">
                                        <Math>r#"$0.001$"#</Math>
                                        r#".)"#
                                    </span>
                                </span>
                            </Indent>
                        </Paragraph>
                    </Solution>
                </Paragraph>
            </Exercise>
        </Exercises>
    }
    