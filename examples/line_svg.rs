fn main() {
    let l1 = plotlib::line::Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(plotlib::style::LineStyle::new().colour("burlywood"));
    let v = plotlib::view::ContinuousView::new().add(&l1);
    plotlib::page::Page::single(&v)
        .save("line.svg")
        .expect("saving svg");
}
