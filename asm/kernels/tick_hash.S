/*
 * Isolated x86_64 SysV checksum mix experiment.
 * Not linked into the default build.
 *
 * uint64_t xenor_tick_hash_mix(uint64_t value);
 */

    .text
    .globl xenor_tick_hash_mix
    .type xenor_tick_hash_mix, @function
xenor_tick_hash_mix:
    movq %rdi, %rax
    xorq $0x9e3779b97f4a7c15, %rax
    rorq $17, %rax
    imulq $0xbf58476d1ce4e5b9, %rax, %rax
    xorq %rdi, %rax
    ret
