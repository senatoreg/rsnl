#[allow(dead_code)]
#[allow(non_camel_case_types)] 
#[link(name="nl-3")]
extern "C" {
	fn nl_socket_alloc() -> *const nl_sock;
	fn nl_socket_free(socket: *const nl_sock);
	fn nl_socket_get_fd(socket: *const nl_sock) -> i32;
	fn nl_socket_set_buffer_size(socket: *const nl_sock, rxbuf: int, txbuf: int) -> i32;
	fn nl_connect(socket: *const nl_sock, protocol: u32) -> i32;
	fn nl_close(socket: *const nl_sock);
}

struct nl_sock;

pub struct socket {
	ptr: *const nl_sock,
}

pub enum NetlinkProtocol {
	route,
	unused,
	usersock,
	firewall,
	sock_diag,
	nflog,
	xfrm,
	selinux,
	iscsi,
	audit,
	fib_lookup,
	connector,
	netfilter,
	ip6_fw,
	dnrtmsg,
	kobject_uevent,
	DMEVENTS,
	scsitransport,
	ecryptfs,
	rdma,
	crypto
}

impl socket {
	pub fn new() -> socket {
		unsafe {
			let nlptr = nl_socket_alloc();
			socket {
				ptr: nlptr,
			}
		}
	}

	pub fn free(&self) {
		unsafe {
			nl_socket_free(self.ptr)
		}
	}

	pub fn set_buffer_size(&self, rxbuf: int, txbuf: int) -> i32 {
		unsafe {
			nl_socket_set_buffer_size(self.ptr, rxbuf, txbuf)
		}
	}

	pub fn connect(&self, protocol: NetlinkProtocol) -> i32 {
		unsafe { nl_connect(self.ptr, protocol as u32) }
	}

	pub fn close(&self) {
		unsafe{ nl_close(self.ptr) }
	}

	pub fn get_fd(&self) -> i32 {
		unsafe { nl_socket_get_fd(self.ptr) }
	}
}