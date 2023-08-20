use std::process::Command;

fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // choose whether to start UEFI or BIOS image
    let uefi = false;

    let mut cmd = Command::new("qemu-system-x86_64");
    if uefi {
	cmd.args(["-bios".into(), ovmf_prebuilt::ovmf_pure_efi()]);
	cmd.args(["-drive", &format!("format=raw,file={uefi_path}")]);
    } else {
	cmd.args(["-drive", &format!("format=raw,file={bios_path}")]);
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
