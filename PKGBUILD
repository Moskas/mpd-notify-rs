pkgname="mpd-notification-rs"
_pkgname="mpd-notify-rs"
pkgver=0.1.1
pkgrel=1
pkgdesc="MPD Notification daemon"
arch=("x86_64" "aarch64")
url="https://github.com/Moskas/mpd-notification-rs"
license=("MIT")
depends=()
optdepends=('dunst: notification daemon')
makedepends=("rust" "cargo" "git" "libnotify")
source=("$_pkgname::git+https://github.com/Moskas/mpd-notification-rs.git")
sha256sums=("SKIP")

pkgver() {
    cd "$_pkgname"
    echo "$pkgver"
    #echo "$(grep '^version =' Cargo.toml|head -n1|cut -d\" -f2).$(git rev-list --count HEAD).g$(git rev-parse --short HEAD)"
}

 build() {
  cd $_pkgname
  cargo build --release --locked --all-features --target-dir=target
 }

check() {
   cd $_pkgname
   cargo test --release --locked --target-dir=target
 }

package() {
    cd "$srcdir/$_pkgname"
    install -Dm755 target/release/$_pkgname "$pkgdir/usr/bin/$_pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
