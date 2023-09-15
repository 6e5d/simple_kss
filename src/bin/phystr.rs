use std::sync::mpsc::channel;

use polysplit::poly_host::PolyHost;
use paraserv::param_server::ParamServer;
use phystr::psgen::Psgen;

fn main() {
	let path = std::env::var("XDG_CACHE_HOME").unwrap() + "/simple_kss.sock";
	let _ = std::fs::remove_file(&path);
	let (send, recv) = channel();
	let sserv = ParamServer::new(&path, send);
	std::thread::spawn(move || sserv.run());
	let psgen = Psgen::new(recv);
	let ph = PolyHost::new(Box::new(psgen));
	eprintln!("init ready");
	ph.run();
}
