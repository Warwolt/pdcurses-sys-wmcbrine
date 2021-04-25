extern crate pdcurses;

fn main() {
    unsafe {
        pdcurses::initscr();
        pdcurses::printw(b"press any key to continue . . .\0".as_ptr() as *const i8);
        pdcurses::refresh();
        pdcurses::getchar();
        pdcurses::endwin();
    }
}
