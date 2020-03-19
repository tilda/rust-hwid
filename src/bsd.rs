// NOTE: BSD **DOES NOT** have real HWIDs.
// Instead, at least on FreeBSD, the equivalent is a "Host ID"
// which is a 32-bit identifier that is normally a DARPA Internet address (I assume that means just an IP).
// The relevant function in C is sysctl() (or if deprecated can be used, gethostid().)
// TODO: Find the appropriate function that's the equivalent of FBSD's standard library function???
// TODO: If Host IDs aren't in some of the BSD distros, find their equivalent files/stdlib functions
mod bsd {
    pub fn getID() {

    }
}