use astrology_draw_svg::chart_html;
fn main() {
    //println!("{}", chart(550.0, "/Users/stephanebressani/Svg/"));
    chart_html(550.0, "/Users/stephanebressani/Svg/index.html")
        .expect("Error generate svg html");
}
