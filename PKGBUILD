# Maintainer: Vinicius Mayrink <vncsmyrnk@gmail.com>
pkgname=todayiwill
pkgver=0.5.3
pkgrel=1
pkgdesc="A CLI reminder app that offers a simple yet powerful solution to enhance productivity and ensure that you stay on top of your daily responsibilities"
arch=('x86_64')
url="https://github.com/vncsmyrnk/todayiwill"
license=('GPL3')
source=("https://github.com/vncsmyrnk/todayiwill/releases/download/v${pkgver}/todayiwill")
sha256sums=('SKIP')

package() {
    cd "$srcdir"
    install -Dm755 "todayiwill" "$pkgdir/usr/bin/todayiwill"
}
