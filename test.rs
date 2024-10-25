
    view! {
        <Image  src="/images/svg_ch4_polaroids.svg" >""</Image><Section  ><Paragraph  ><span class="text"><Span bold=true>r#"Definitions."#</Span>r#"
 The "#<Span italic=true>r#"derivative"#</Span>r#" of a function"#</span></Paragraph><Paragraph  ><MathBlock>r#"$$
 f : \rr \ra \rr 
 $$"#</MathBlock></Paragraph><Paragraph  ><span class="text">r#"is a (new) function"#</span></Paragraph><Paragraph  ><MathBlock>r#"$$
 f' : \rr \ra \rr
 $$"#</MathBlock></Paragraph><Paragraph  ><span class="text">r#"that gives the slope of "#<Math>r#"$f$"#</Math>r#" at each point. 
 In other words"#</span></Paragraph><Paragraph  ><MathBlock>r#"$$
 f'(a)
 $$"#</MathBlock></Paragraph><Paragraph  ><span class="text">r#"is the slope of the graph "#<Math>r#"$y = f(x)$"#</Math>r#" at
 "#<span class="nobreak"><Math>r#"$x = a$"#</Math>r#"."#</span>r#" And—surprise!—each pair of graphs
 above is a pair of the form "#<Math>r#"$y = f(x)$"#</Math>r#" "#<span class="nobreak">r#"["#<Math>r#"$=$"#</Math>r#"
"#</span>r#" “before”], "#<Math>r#"$y = f'(x)$"#</Math>r#" ["#<Math>r#"$=$"#</Math>r#" “after”]. (Meaning,
 the “after” graph records the slope of the 
 “before” graph.) E.g.:"#</span></Paragraph><Paragraph  ><Image  src="/images/svg_ch4_explanation1.svg" >""</Image></Paragraph><Paragraph  ><span class="text">r#"Note that "#<Math>r#"$f'\!$"#</Math>r#" (read “"#<Math>r#"$f$"#</Math>r#" prime”) remains 
 undefined where "#<Math>r#"$y = f(x)$"#</Math>r#" has a sharp “corner” 
 and no well-defined slope. By opposition, if 
 there is a well-defined tangent line to 
 "#<Math>r#"$y = f(x)$"#</Math>r#" at "#<Math>r#"$x = a$"#</Math>r#" the slope of this
 tangent line supplies the value of "#<span class="nobreak"><Math>r#"$f'(a)$"#</Math>r#":"#</span></span></Paragraph><Paragraph  ><Image  src="/images/svg_ch4_explanation2.svg" >""</Image><ImageRight  y="top" offset_y="1em" offset_x="-5em" src="/images/svg_ch4_explanation2_cloud.svg" >""</ImageRight></Paragraph><Paragraph  ><span class="text">r#"In fact,
 we can
 succinctly describe the derivative by..."#</span></Paragraph><Paragraph  ><MathBlock>r#"$$
 f'(a) = \te{[slope of tangent line to $y = f(x)$ at $x = a$]}
 $$"#</MathBlock></Paragraph><Paragraph  ><span class="text">r#"...with the understanding that "#<Math>r#"$f'(a)$"#</Math>r#" is 
 undefined if a tangent line does not exist 
 or if the tangent is vertical. But for one 
 last asterisk—and speaking of the existence,
 or not, of the tangent—note that the endpoint
 of a curve does not count as having a tangent,
 and therefore leaves a missing value for the 
 derivative:"#</span></Paragraph><Paragraph  ><Image  src="/images/svg_ch4_explanation_one_sided_tangent.svg" >""</Image></Paragraph><Paragraph  ><span class="text">r#"(In other words, “half-tangents” do not actually 
 count as tangents.) "#</span></Paragraph></Section>
    }
    