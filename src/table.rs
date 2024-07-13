use crate::counter::Counter;

pub fn draw_table(counts: &Counter) {
    let headers = ["Files", "Lines", "Code", "Comments", "Blanks"];

    fn print_splitter() {
        println!(
            "{:->7}{:->12}{:->12}{:->12}{:->12}",
            "-", "-", "-", "-", "-"
        );
    }

    // 打印表头
    print_splitter();
    println!(
        "{:>6}{:>12}{:>12}{:>12}{:>12}",
        headers[0], headers[1], headers[2], headers[3], headers[4]
    );
    print_splitter();

    // 打印数据
    println!(
        "{:>6}{:>12}{:>12}{:>12}{:>12}",
        counts.files,
        counts.lines(),
        counts.code,
        counts.comments,
        counts.blanks,
    );
    print_splitter();
}
