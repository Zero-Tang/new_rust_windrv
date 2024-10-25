use std::{env::*, fs::*, io::*, process::*, time::*};

use indoc::indoc;

const HELLO_TEXT:&str = indoc! {"
Welcome to Rust!

This wizard will create a crate that builds a Windows Driver.
You only need to follow the wizard's guide to create a new crate.

"};

const CARGO_TOML_ADDITION:&str = indoc! {"

[lib]
crate-type = [\"cdylib\"]

[profile.dev]
panic = \"abort\"

[profile.release]
panic = \"abort\"

[package.metadata.wdk.driver-model]
driver-type = "};

const MAKEFILE_TOML_CONTENT:&str = indoc! {"
extend = \"target/rust-driver-makefile.toml\"

[config]
load_script = '''
#!@rust
//! ```cargo
//! [dependencies]
//! wdk-build = \"0.3.0\"
//! ```
#![allow(unused_doc_comments)]

wdk_build::cargo_make::load_rust_driver_makefile()?
'''
"};

const CONFIG_TOML_CONTENT:&str = indoc! {"
[build]
rustflags = [\"-C\", \"target-feature=+crt-static\"]
"};

const BUILD_RS_CONTENT:&str = indoc! {"
fn main() -> Result<(), wdk_build::ConfigError> {
   wdk_build::configure_wdk_binary_build()
}
"};

const LIB_RS_CONTENT:&str = indoc! {"// New Windows Driver Crate
#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

use wdk_alloc::WdkAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

use wdk_sys::*;

#[export_name = \"DriverEntry\"]
pub unsafe extern \"system\" fn driver_entry(_driver: PDRIVER_OBJECT, _registry_path: PCUNICODE_STRING) -> NTSTATUS
{
	STATUS_SUCCESS
}
"};

macro_rules! INX_CONTENT {
	() => {
		indoc! {"
;===================================================================
; Copyright (c) [Year], [Author Name]
;
;Module Name:
;    {0}.inf
;===================================================================

[Version]
Signature   = \"$WINDOWS NT$\"
Class       = SoftwareDevice
ClassGuid   = {{62f9c741-b25a-46ce-b54c-9bccce08b6f2}}
Provider    = %ProviderString%
PnpLockDown = 1

[DestinationDirs]
DefaultDestDir = 13

[SourceDisksNames]
1 = %DiskId1%,,,\"\"

[SourceDisksFiles]
{0}.sys  = 1,,


; ================= Install section =================

[Manufacturer]
%StdMfg%=Standard,NT$ARCH$.10.0...16299

[Standard.NT$ARCH$.10.0...16299]
%{1}.DeviceDesc%={1}_Device, root\\{0}

[{1}_Device.NT$ARCH$]
CopyFiles=Drivers_Dir

[Drivers_Dir]
{0}.sys

; ================= Service installation =================
[{1}_Device.NT$ARCH$.Services]
AddService = {0}, %SPSVCINST_ASSOCSERVICE%, {1}_Service_Inst

[{1}_Service_Inst]
DisplayName    = %{1}.SVCDESC%
ServiceType    = 1               ; SERVICE_KERNEL_DRIVER
StartType      = 3               ; SERVICE_DEMAND_START
ErrorControl   = 1               ; SERVICE_ERROR_NORMAL
ServiceBinary  = %13%\\{0}.sys

; ================= Strings =================
[Strings]
SPSVCINST_ASSOCSERVICE = 0x00000002
ProviderString         = \"SampleProvider\"
StdMfg                 = \"(Standard system devices)\"
DiskId1                = \"sample\"
ClassName              = \"sample\"
{1}.DeviceDesc      = \"sample device\"
{1}.SVCDESC         = \"sample service\"
"}
	};
}

fn main()
{
	print!("{}", HELLO_TEXT);
	let mut editing:bool = true;
	let mut crate_name = String::new();
	let mut driver_type = String::new();
	let mut vcs_type = String::new();
	while editing
	{
		// Receive the crate name.
		while crate_name.is_empty()
		{
			println!("What's the name of your new Windows Driver crate? Please name your driver in snake_case.");
			if let Err(e) = stdin().read_line(&mut crate_name)
			{
				panic!("Failed to get the name of the new Windows Driver crate! Reason: {e}");
			}
			crate_name = String::from(crate_name.trim()).to_lowercase();
		}
		// Receive the driver type.
		while driver_type.is_empty()
		{
			println!("What's your driver type? Valid Options: [WDM | KMDF | UMDF]");
			if let Err(e) = stdin().read_line(&mut driver_type)
			{
				panic!("Failed to get the driver type! Reason: {e}");
			}
			driver_type = String::from(driver_type.trim()).to_uppercase();
			match driver_type.as_str()
			{
				"WDM" | "KMDF" | "UMDF" =>
				{}
				_ =>
				{
					if driver_type.is_empty()
					{
						continue;
					}
					println!("Unrecognized driver type: {}!", driver_type);
					driver_type = String::new();
				}
			}
		}
		// Receive the VCS type.
		while vcs_type.is_empty()
		{
			println!("What's your VCS type? For valid options, see: https://doc.rust-lang.org/cargo/commands/cargo-new.html#new-options");
			println!("Recommended options: [git | none]");
			if let Err(e) = stdin().read_line(&mut vcs_type)
			{
				panic!("Failed to get the VCS type! Reason: {e}");
			}
			vcs_type = String::from(vcs_type.trim()).to_lowercase();
			// Note: we will not check validity of VCS type!
		}
		// Confirmation
		println!("\nAre you sure?\nConfirm the following configurations:");
		println!("Crate Name: {}", crate_name);
		println!("Driver Type: {}", driver_type);
		println!("Version-Control System: {}", vcs_type);
		println!("\nType 1 to confirm. Type 2 to retry. Type 3 to quit. (Default: 1 - Confirm)");
		let mut confirmation = String::new();
		loop
		{
			if let Err(e) = stdin().read_line(&mut confirmation)
			{
				panic!("Failed to confirm! Reason: {e}");
			}
			confirmation = String::from(confirmation.trim());
			if confirmation.is_empty()
			{
				editing = false;
				break;
			}
			else
			{
				let c = confirmation.parse::<u32>();
				match c
				{
					Ok(n) =>
						match n
						{
							1 =>
							{
								editing = false;
								break;
							}
							2 => break,
							3 =>
							{
								exit(2);
							}
							_ =>
							{
								println!("Unrecognized input!");
								continue;
							}
						},
					Err(e) => println!("{e}")
				}
			}
		}
	}
	// The wizard has collected all needed info.
	let timer = SystemTime::now();
	// Use a macro to simplify error-handling while creating the processes.
	macro_rules! handle_process_output {
		($proc_out:expr) => {
			match $proc_out
			{
				Ok(out) =>
				{
					if out.code().unwrap() != 0
					{
						panic!("Cargo returned non-zero status! {}", out);
					}
				}
				Err(e) => panic!("Failed to execute cargo! Reason: {e}")
			}
		};
	}
	// Create the crate.
	let cargo_out = Command::new("cargo").args(["new", crate_name.as_str(), "--lib", "--vcs", vcs_type.as_str()]).status();
	handle_process_output!(cargo_out);
	// Switch directory.
	let r = set_current_dir(crate_name.as_str());
	if let Err(e) = r
	{
		panic!("Failed to switch directory! Reason: {}", e);
	}
	// Add dependencies.
	let cargo_out = Command::new("cargo").args(["add", "--build", "wdk-build"]).status();
	handle_process_output!(cargo_out);
	let cargo_out = Command::new("cargo").args(["add", "wdk", "wdk-sys", "wdk-alloc", "wdk-panic"]).status();
	handle_process_output!(cargo_out);
	// Use a macro to help handle file errors.
	macro_rules! panic_if_err {
		($r:expr, $n:expr) => {
			if let Err(e) = $r
			{
				panic!("Failed to {}! Reason: {e}", $n);
			}
		};
	}
	// Setup Cargo.toml
	let r = File::options().append(true).open("Cargo.toml");
	match r
	{
		Ok(mut f) =>
		{
			let r = f.write_all(CARGO_TOML_ADDITION.as_bytes());
			panic_if_err!(r, "write to Cargo.toml");
			let r = f.write_all(format!("\"{}\"\n", driver_type).as_bytes());
			panic_if_err!(r, "write to Cargo.toml");
			let r = f.sync_all();
			panic_if_err!(r, "sync Cargo.toml");
		}
		Err(e) => panic!("Failed to open Cargo.toml! Reason: {e}")
	}
	// Setup Makefile.toml
	let r = File::create("Makefile.toml");
	match r
	{
		Ok(mut f) =>
		{
			let r = f.write_all(MAKEFILE_TOML_CONTENT.as_bytes());
			panic_if_err!(r, "write to Makefile.toml");
			let r = f.sync_all();
			panic_if_err!(r, "sync Makefile.toml");
		}
		Err(e) => panic!("Failed to create Makefile.toml! Reason: {e}")
	}
	// Setup .cargo/config.toml
	let r = create_dir(".cargo");
	panic_if_err!(r, "create .cargo directory!");
	let r = File::create(".cargo/config.toml");
	match r
	{
		Ok(mut f) =>
		{
			let r = f.write_all(CONFIG_TOML_CONTENT.as_bytes());
			panic_if_err!(r, "write to .cargo/config.toml");
			let r = f.sync_all();
			panic_if_err!(r, "sync .cargo/config.toml");
		}
		Err(e) => panic!("Failed to create .cargo/config.toml! Reason: {e}")
	}
	// Setup build.rs
	let r = File::create("build.rs");
	match r
	{
		Ok(mut f) =>
		{
			let r = f.write_all(BUILD_RS_CONTENT.as_bytes());
			panic_if_err!(r, "write to build.rs");
			let r = f.sync_all();
			panic_if_err!(r, "sync build.rs");
		}
		Err(e) => panic!("Failed to create build.rs! Reason: {e}")
	}
	// Setup lib.rs
	let r = File::create("src/lib.rs");
	match r
	{
		Ok(mut f) =>
		{
			let r = f.set_len(0);
			panic_if_err!(r, "clear src/lib.rs");
			let r = f.write_all(LIB_RS_CONTENT.as_bytes());
			panic_if_err!(r, "write to src/lib.rs");
			let r = f.sync_all();
			panic_if_err!(r, "sync src/lib.rs");
		}
		Err(e) => panic!("Failed to open src/lib.rs! Reason: {e}")
	}
	// Setup .inx
	let r = File::create(format!("{}.inx", crate_name.as_str()).as_str());
	match r
	{
		Ok(mut f) =>
		{
			let crate_name_lower = crate_name.to_lowercase();
			let crate_name_upper = crate_name.to_uppercase();
			let r = write!(f, INX_CONTENT!(), crate_name_lower, crate_name_upper);
			panic_if_err!(r, "write to .inx file");
			let r = f.sync_all();
			panic_if_err!(r, "sync .inx file");
		}
		Err(e) => panic!("Failed to create {}.inx! Reason: {e}", crate_name)
	}
	println!("Wizard has completed creating a new Windows Driver crate!");
	println!("You will need to manually execute `cargo make` to get started!");
	let r = timer.elapsed();
	match r
	{
		Ok(elapsed) => println!("{} seconds elapsed in creating new crate!", elapsed.as_secs_f64()),
		Err(e) => println!("Timer failed! Reason: {e}")
	}
	// Pause the program.
	Command::new("cmd").args(["/c", "pause"]).status().unwrap();
}
