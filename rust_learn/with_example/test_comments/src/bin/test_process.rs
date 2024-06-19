use std::{io::Read, process::Command};
use std::process::Stdio;

fn main() {
    let output = Command::new("tail")
		// .arg("-f")
        .arg("hello.txt")
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();
	let mut buf = [0; 20];
	let mut out = output.stdout.unwrap();
	let _ = out.read(&mut buf);
	println!("{:?}", buf);
	// let mut s = String::new();
	// let _ = output.stdout.unwrap().read(buf);
	// println!("{}", s);
	// loop {
	
		//  if let Ok(s) = out.read(&mut buf[..]) {
		// 	println!("{} {:?}", s, &buf[..s]);
		// 	// if s == 0 {
		// 	// 	break;
		// 	// }
		// } else {
		// }
	// }
        // .output().unwrap_or_else(|e| {
        //     panic!("failed to execute process: {}", e)
    // });
    // if output.status.success() {
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     print!("rustc succeeded and stdout was:\n{}", s);
    // } else {
    //     let s = String::from_utf8_lossy(&output.stderr);
    //     print!("rustc failed and stderr was:\n{}", s);
    // }
}

// use std::process::Command;
// fn main() {
//     let output = Command::new("pwd")
//         // .arg("--version")
//         .output().unwrap_or_else(|e| {
//             panic!("failed to execute process: {}", e)
//     });
//     if output.status.success() {
//         let s = String::from_utf8_lossy(&output.stdout);
//         print!("rustc succeeded and stdout was:\n{}", s);
//     } else {
//         let s = String::from_utf8_lossy(&output.stderr);
//         print!("rustc failed and stderr was:\n{}", s);
//     }
// }