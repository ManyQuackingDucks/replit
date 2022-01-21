{ pkgs }: {
	deps = [
        pkgs.rustup
        pkgs.rust-analyzer
        pkgs.openssl
        pkgs.pkgconfig
	];
}