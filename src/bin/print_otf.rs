use std::error::Error;

use otf::Font;

fn main() -> Result<(), Box<dyn Error>> {
    let font = Font::from_file("./SourceHanSansCN-Regular.otf")?;
    println!("{:#?}", font);

    Ok(())
}
