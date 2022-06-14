use tonic_build;

fn main() {
    tonic_build::configure()
        .compile(&["model/account.proto", "model/auth.proto", "model/transaction.proto", "model/user.proto"], &["model"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
