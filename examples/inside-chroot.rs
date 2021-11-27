use inside_chroot::inside_chroot;

fn main() {
    std::process::exit(match inside_chroot() {
        Ok(chroot) => {
            if chroot {
                println!("currently inside chroot");
                0
            } else {
                println!("currently not inside chroot");
                1
            }
        }
        Err(e) => {
            println!("error {}", e);
            -1
        }
    })
}
