use qrcode::render::unicode;
use qrcode::QrCode;

fn main() {
    let code = QrCode::new("https://github.com/TerakomariGandesblood").unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}
