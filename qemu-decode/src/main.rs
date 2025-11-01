use anyhow::Context as _;
use defmt_decoder::DecodeError;
use std::{
    io::Read as _,
    process::{Child, Command},
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        anyhow::bail!("usage: qemu-decode [ELF_FILE]");
    }

    let elf_path = &args[1];
    let elf: Vec<u8> =
        std::fs::read(elf_path).with_context(|| format!("Failed to read ELF file: {elf_path}"))?;

    let table: defmt_decoder::Table = defmt_decoder::Table::parse(&elf)
        .context("Failed to parse ELF")?
        .context(".defmt section missing from ELF")?;

    #[rustfmt::skip]
    const QEMU_ARGS: &[&'static str] = &[
        "-cpu", "cortex-m4",
        "-machine", "lm3s6965evb",
        "-nographic",
        "-monitor", "none",
        "-semihosting-config", "enable=on,target=native",
        "-kernel",
    ];

    let (mut reader, writer) = std::io::pipe().unwrap();

    let mut qemu: Child = Command::new("qemu-system-arm")
        .args(QEMU_ARGS)
        .arg(elf_path)
        .stdout(writer)
        .spawn()
        .context("Failed to spawn QEMU process")?;

    let mut decoder = table.new_stream_decoder();
    let mut buffer = [0u8; 1024];

    loop {
        match qemu.try_wait().context("Failed to wait for QEMU")? {
            Some(status) => {
                if let Some(code) = status.code() {
                    std::process::exit(code);
                } else {
                    return Ok(());
                }
            }
            None => {
                let len = reader.read(&mut buffer)?;
                decoder.received(&buffer[..len]);

                match decoder.decode() {
                    Ok(frame) => println!("{}", frame.display(false)),
                    Err(DecodeError::UnexpectedEof) => continue,
                    Err(e) => Err(e).context("Error decoding defmt data")?,
                }
            }
        }
    }
}
