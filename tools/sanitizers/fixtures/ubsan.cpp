// Isolated native C++ signed-overflow control; no Rust instrumentation claim.
#include <climits>
#include <cstdio>

int main(int argc, char**) {
    volatile int increment = argc;
#ifdef SEEDED_DEFECT
    volatile int base = INT_MAX;
#else
    volatile int base = 1;
#endif
    const int result = base + increment;
    if (result != 2) { return 3; }
    std::puts("SANITIZER_HEALTHY");
    return 0;
}
