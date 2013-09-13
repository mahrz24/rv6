// Source from xv6 which doesn't work in rust (some linking problem)
// I guess everything in here could also be transfered to rust at some
// point

#include <memory_c.h>

typedef unsigned int pde_t;
pde_t entrypgdir[];
pde_t *kpgdir;

extern char end[];
extern char data[];

void * get_end() {
  return (void*)end;
}

void * get_data() {
  return (void*)data;
}



// Boot page table used in entry.S and entryother.S.
// Page directories (and page tables), must start on a page boundary,
// hence the "__aligned__" attribute.
// Use PTE_PS in page directory entry to enable 4Mbyte pages.
__attribute__((__aligned__(PGSIZE)))
pde_t entrypgdir[NPDENTRIES] = {
  // Map VA's [0, 4MB) to PA's [0, 4MB)
  [0] = (0) | PTE_P | PTE_W | PTE_PS,
  // Map VA's [KERNBASE, KERNBASE+4MB) to PA's [0, 4MB)
  [KERNBASE>>PDXSHIFT] = (0) | PTE_P | PTE_W | PTE_PS,
};