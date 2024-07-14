use crate::counter::Counter;

pub fn draw_table(counts: &Counter) {
    let headers = ["Files", "Lines", "Code", "Comments", "Blanks"];

    fn print_splitter() {
        println!("{:->59}", "-");
    }

    // 打印表头
    print_splitter();
    println!(
        "{:>6}{:>13}{:>13}{:>13}{:>13}",
        headers[0], headers[1], headers[2], headers[3], headers[4]
    );
    print_splitter();

    // 打印数据
    println!(
        "{:>6}{:>13}{:>13}{:>13}{:>13}",
        counts.files,
        counts.lines(),
        counts.code,
        counts.comments,
        counts.blanks,
    );
    print_splitter();
}
