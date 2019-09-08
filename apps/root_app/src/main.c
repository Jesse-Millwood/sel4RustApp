
#include <stdio.h>
#include <sel4/sel4.h>
#include <sel4platsupport/bootinfo.h>
#include <utils/util.h>

/* Calling Rust Functions */
extern uint32_t rust_routine(uint32_t a, uint32_t b);

int main(int argc, char *argv[]) {

    seL4_BootInfo *info = platsupport_get_bootinfo();
    size_t initial_cnode_object_size = BIT(info->initThreadCNodeSizeBits);
    size_t num_initial_cnode_slots = initial_cnode_object_size / (1u << seL4_SlotBits);

    /* Make call to rust function */
    uint32_t a = 10;
    uint32_t b = 20;
    uint32_t c = rust_routine(a, b);
    printf("Rust Routine: %u + %u = %u\n", a, b, c);

    printf("Suspending current thread\n");
    seL4_TCB_Suspend(seL4_CapInitThreadTCB);
    ZF_LOGF("Failed to suspend current thread\n");

    return 0;
}
