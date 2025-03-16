#include <stdlib.h>
#include <limits.h>
#include <sys/sysctl.h>
#include <sys/mount.h>
#include <mach/mach_init.h>
#include <mach/mach_host.h>
#include <mach/host_info.h>
#include <TargetConditionals.h>
#if TARGET_OS_MAC
#include <libproc.h>
#endif
#include <mach/processor_info.h>
#include <mach/vm_map.h>