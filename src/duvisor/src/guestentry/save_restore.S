.macro SAVE_HOST_CTX reg1
	addi \reg1, \reg1, HOST_GP
    SAVE_GP \reg1
    addi \reg1, \reg1, -HOST_GP
.endm

.macro RESTORE_HOST_CTX reg1
	addi \reg1, \reg1, HOST_GP
    RESTORE_GP \reg1
    addi \reg1, \reg1, -HOST_GP
.endm

.macro SAVE_GUEST_CTX reg1
	addi \reg1, \reg1, GUEST_GP
    SAVE_GP \reg1
    addi \reg1, \reg1, -GUEST_GP
.endm

.macro RESTORE_GUEST_CTX reg1
	addi \reg1, \reg1, GUEST_GP
    RESTORE_GP \reg1
    addi \reg1, \reg1, -GUEST_GP
.endm

.macro SAVE_GP reg1
    sd x1, GP_X1(\reg1)
    sd x2, GP_X2(\reg1)
    sd x3, GP_X3(\reg1)
    sd x4, GP_X4(\reg1)
    sd x5, GP_X5(\reg1)
    sd x6, GP_X6(\reg1)
    sd x7, GP_X7(\reg1)
    sd x8, GP_X8(\reg1)
    sd x9, GP_X9(\reg1)
    sd x10, GP_X10(\reg1)
    sd x11, GP_X11(\reg1)
    sd x12, GP_X12(\reg1)
    sd x13, GP_X13(\reg1)
    sd x14, GP_X14(\reg1)
    sd x15, GP_X15(\reg1)
    sd x16, GP_X16(\reg1)
    sd x17, GP_X17(\reg1)
    sd x18, GP_X18(\reg1)
    sd x19, GP_X19(\reg1)
    sd x20, GP_X20(\reg1)
    sd x21, GP_X21(\reg1)
    sd x22, GP_X22(\reg1)
    sd x23, GP_X23(\reg1)
    sd x24, GP_X24(\reg1)
    sd x25, GP_X25(\reg1)
    sd x26, GP_X26(\reg1)
    sd x27, GP_X27(\reg1)
    sd x28, GP_X28(\reg1)
    sd x29, GP_X29(\reg1)
    sd x30, GP_X30(\reg1)
    sd x31, GP_X31(\reg1)
.endm

.macro RESTORE_GP reg1
    ld x1, GP_X1(\reg1)
    ld x2, GP_X2(\reg1)
    ld x3, GP_X3(\reg1)
    ld x4, GP_X4(\reg1)
    ld x5, GP_X5(\reg1)
    ld x6, GP_X6(\reg1)
    ld x7, GP_X7(\reg1)
    ld x8, GP_X8(\reg1)
    ld x9, GP_X9(\reg1)
    ld x11, GP_X11(\reg1)
    ld x12, GP_X12(\reg1)
    ld x13, GP_X13(\reg1)
    ld x14, GP_X14(\reg1)
    ld x15, GP_X15(\reg1)
    ld x16, GP_X16(\reg1)
    ld x17, GP_X17(\reg1)
    ld x18, GP_X18(\reg1)
    ld x19, GP_X19(\reg1)
    ld x20, GP_X20(\reg1)
    ld x21, GP_X21(\reg1)
    ld x22, GP_X22(\reg1)
    ld x23, GP_X23(\reg1)
    ld x24, GP_X24(\reg1)
    ld x25, GP_X25(\reg1)
    ld x26, GP_X26(\reg1)
    ld x27, GP_X27(\reg1)
    ld x28, GP_X28(\reg1)
    ld x29, GP_X29(\reg1)
    ld x30, GP_X30(\reg1)
    ld x31, GP_X31(\reg1)
.endm
