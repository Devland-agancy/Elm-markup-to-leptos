pub mod element_text;
pub mod transform;
use leptos::html::Tr;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use transform::Transformer;

fn main() {
    let mut transformer: Transformer = Transformer::new(
        vec!["img", "SectionDivider"],
        vec!["Paragraphs", "Example", "Section", "Solution"],
        vec!["Example"],
        "Paragraph",
        vec![
            "Image",
            "DisplayImage",
            "Pause",
            "InlineImage",
            "MathBlock",
            "Table",
        ],
        vec![
            "Paragraphs",
            "Paragraph",
            "Example",
            "Section",
            "tr",
            "Table",
            "Solution",
        ],
    );

    let pre = transformer.pre_process_exercises(
        r#"
|> Section

    ok 

|> Exercise

    |> ExerciseQuestion

        *Exercise 1. *
        True or false: Lines of slope $ -1 \over 2 $ are perpendicular to lines of slope $2$.

    |> Solution

        This is true, as illustrated by the following pair of lines:
        |> Image
            src="/images/55.svg"
            container_classes="relative w-fit"

            |> ImageRight
                src="/images/56.svg"
                width=340
                x_anchor="8%"
                y_anchor="13rem"

            |> ImageLeft
                src="/images/57.svg"
                width=340
                x_anchor="30%"
                y_anchor="16rem"

        In more detail, the two triangles are related by
        a $90^\circ$ rotation and so, likewise, are the lines defined
        by their hypotenuses!

        _Note 1._ 
        More generally, a line of slope $p$ is perpendicular to a line
        of slope $-1/p$, for all $p \ne 0$. By a similar drawing, for $p > 0$:

        |> Image
            src="/images/58.svg"
            container_classes="relative w-fit"

            |> ImageRight
                src="/images/59.svg"
                width=340
                x_anchor="8%"
                y_anchor="15rem"

            |> ImageLeft
                src="/images/60.svg"
                width=340
                x_anchor="25%"
                y_anchor="16rem"

        If you're curious, a drawing for the case $p < 0$...

        |> Image
            src="/images/61.svg"

        ...is like so, but it doesn't really add anything new. 

|> Exercise

    |> ExerciseQuestion

        *Exercise 2. *
        Find the general equation of a line of slope 
        $p$ passing through a point $(x_0, y_0)$.
        (Hint: Start from the slope formula.) 

    |> Solution

        A point $(x,y) \ne (x_0,y_0)$ is on the line of
        slope $p$ if and only if


        $$ p = {y - y_0 \over x - x_0} $$
        |> ImageLeft
            src="/images/62.svg"
            width=800
            x_anchor="6.5rem"
            y_anchor="-5.7rem"

        because

        $$ \,\,{y - y_0 \over x - x_0} $$

        is the slope of the line segment from $(x_0,y_0)$ 
        to $(x,y)$, and it is necessary and
        sufficient for this segment to have slope $p$ in order
        for the point $(x,y)$ to be on the line!

        Unfortunately, the equation

        $$ p = {y - y_0 \over x - x_0} $$

        is not an entirely satisfactory answer, because the
        point $(x,y) = (x_0,y_0)$ itself does not satisfy the
        equation. (We find

        $$ p = {0 \over 0} $$

        if we plug in $x = x_0$, $y = y_0$, which is not a valid
        equality because the right-hand side is an undefined quantity.)

        Instead, multiplying

        $$
        p = {y - y_0 \over x - x_0}
        $$

        on both sides by $x-x_0$, we find the fraction-less equation

        $$
        p(x-x_0) = y-y_0
        $$

        which is satisfied by the point $(x,y) = (x_0,y_0)$ as well as
        by every other point on the line.
        This can be a final answer, and, pleasingly, has the form

        $$ \te{“}\te{slope} \times \te{run} = \te{rise}\te{”} $$
        |> ImageRight
            src="/images/63.svg"
            width=400
            x_anchor="-1.5rem"
            y_anchor="-3.5rem"
            hidden_in_mobile=true
            squiggle_top="36%"
            squiggle_left="-3.8rem"

        which can also make it easy to remember!

        _Note 1._
        The answer we gave is more often written

        $$
        y - y_0 = p(x - x_0)
        $$

        with the two sides of the equation swapped, or

        $$
        y = p(x - x_0) + y_0
        $$

        with $y$ isolated on the left-hand side. From there one
        can also distribute $p(x-x_0)$, obtaining (after putting
        “$-px_0$” last)

        $$
        y = px + y_0 - px_0
        $$

        which has the form

        $$
        y = ax + b
        $$

        with $a = p$, $b = y_0 - px_0$.

|> Exercise

    |> ExerciseQuestion

        *Exercise 3. *
        Plot the vertical velocity of an object a mosquito whose height over time is given by this graph (use the same time interval as the graph):

    |> Image
        src="/images/1.svg"
        container_classes="pt-[22px] pb-[15px]"
        padding_left=90_f64

    |> Solution

        Here is the “official” graph of the (vertical) velocity:

        |> Image
            src="/images/64.svg"
            container_classes="pt-[21px] pb-[20px]"
            padding_left=46_f64

        On each interval, the velocity is rate of change
        of the height, i.e., the _slope_ of the height.
        For example, the rate of change of the height is

        $$
        {1\te{m} \over 1\te{s}} = 1\te{m}/\te{s}
        $$

        between $-4$s and $-3$s, where the mosquito goes up by one meter
        during a one second period, so the vertical velocity is
        1m$/$s for that time interval, etc.

        _Note 1._
        As explained in Chapter 3, an empty circle of this type

        |> Image
            src="/images/65.svg"

        indicates a “missing” value. 
        Specifically, in our case, the vertical velocity is _undefined_
        wherever the graph of the height has a sharp corner.
        (Because the slope of the graph is not well-defined at such corners.)

        _Note 2._ 
        For the time interval from $2$s to $2.5$s, the slope is

        $$ {-2\rt{0.05}\te{m} \over 0.5\rt{0.05}\te{s}} = -\rt{0.07}4\rt{0.1}\te{m}/\te{s} $$

        and similarly for the time interval 
        from $2.5$s to $3$s the slope is

        $$ {2\rt{0.05}\te{m} \over 0.5\rt{0.05}\te{s}} = 4\rt{0.1}\te{m}/\te{s} $$

        because $2/0.5 = 4$.
        (Think: _how many times_ does $0.5$ go into $2$?)

|> Exercise

    |> ExerciseQuestion

        *Exercise 4. *
        Digressing on the second-to-last equation
        in the solution to Exercise 2, explain why

        $$ y_0 - px_0 $$

        is the $y$-intercept of the line of slope $p$ through
        the point $(x_0,y_0)$ by using a drawing and “rise equals
        slope times run”.

    |> Solution
        
        E.g.:

        |> Image
            src="/images/66.svg"

        The rise from the $y$-intercept to $(x_0, y_0)$ is $px_0$, as found
        by “rise equals slope times run”, implying that

        $$
        y_0 - px_0
        $$

        is the $y$-intercept.
        
        _Note 1._ Our drawing makes some implicit
        assumptions, such as $p > 0$ and $x_0 > 0$.
        But

        $$ px_0 $$

        is the rise from the $y$-intercept to $(x_0, y_0)$
        regardless of the sign of $p$ or $x_0$ (because “$x_0$” is
        the run in all cases), making

        $$ y_0 - px_0 $$

        the $y$-intercept in all cases.

    "#
        .to_string(),
    );
    let leptos_code = transformer.transform(pre, 0);
    let mut file = match File::create("src/output.rs") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };
    let file_content = format!(
        r#"
    view! {{
        {}
    }}
    "#,
        leptos_code
    );
    match file.write_all(file_content.as_bytes()) {
        Ok(_) => {
            println!(
            "Data written to file successfully , use leptosfmt to format leptos for better view"
        );
            let _ = Command::new("leptosfmt")
                .arg("src/output.rs")
                .output()
                .expect("Failed to execute command");
        }
        Err(error) => println!("Error writing to file: {}", error),
    }
}
