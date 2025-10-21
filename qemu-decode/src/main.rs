use anyhow::Context as _;
use defmt_decoder::DecodeError;
use std::{
    io::Read as _,
    process::{self, Child, ChildStdout, Command, Stdio},
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

    let mut qemu: Child = Command::new("qemu-system-arm")
        .args(QEMU_ARGS)
        .arg(elf_path)
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn QEMU process")?;

    let mut stdout: ChildStdout = qemu.stdout.take().context("Failed to take QEMU stdout")?;

    let mut decoder = table.new_stream_decoder();

    let mut endbuf: Vec<u8> = Default::default();
    stdout.read_to_end(&mut endbuf)?;
    decoder.received(&endbuf);
    loop {
        match decoder.decode() {
            Ok(frame) => println!("{}", frame.display(false)),
            Err(DecodeError::UnexpectedEof) => break,
            Err(e) => Err(e).context("Error decoding defmt data")?,
        }
    }
    qemu.try_wait().context("Failed to wait for QEMU")?;

    if let Some(exit_status) = qemu.try_wait().context("Failed to wait for QEMU")?
        && let Some(exit_code) = exit_status.code()
    {
        process::exit(exit_code);
    }

    Ok(())
}
