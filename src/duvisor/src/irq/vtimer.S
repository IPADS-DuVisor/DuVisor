.global rdvtimecmp
rdvtimecmp:
    .word 0xe0102577
    ret

.global wrvtimecmp
wrvtimecmp:
    .word 0xe0a01077
    ret

.global rdvtimectl
rdvtimectl:
    .word 0xf0202577
    ret

.global wrvtimectl
wrvtimectl:
    .word 0xf0a01077
    ret