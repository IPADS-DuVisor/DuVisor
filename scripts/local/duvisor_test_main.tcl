proc main_test { } {
    # Test the binary
    expect ":/duvisor#"

    send "./duvisor --smp 4 --block /blk-dev.img\n"
    expect {
            "please set memory size by using --memory or config files." {
        }
        timeout {
            exit -1
        }
    }

    send "./duvisor --memory 128 --block /blk-dev.img\n"
    expect {
            "please set vcpu count by using --smp or config files." {
        }
        timeout {
            exit -1
        }
    }

    send "./duvisor --smp 4 --memory 128 --block /blk-dev.img\n"
    expect {
            "please set kernel image by using --kernel or config files." {
        }
        timeout {
            exit -1
        }
    }

    set timeout 300

    send "./duvisor --smp 1 --initrd ./test-files-duvisor/rootfs-net.img --dtb ./test-files-duvisor/vmlinux.dtb  --kernel ./test-files-duvisor/GuestLinuxImage.ok --memory 1024 --machine duvisor_virt --block /blk-dev.img --vmtap vmtap0 --append 'console=ttyS0 root=/dev/vda rw  console=sbi earlycon=sbi'\n"
    expect {
        "Busybox Rootfs" {
            send "\n ls \n"
            expect {
                "guest-net.sh" {}
            }
        }
        
        timeout {
            exit -1
        }
    }

    send "/guest-net.sh \n ip a \n"
    expect {
        "eth0: <BROADCAST,MULTICAST,UP,LOWER_UP>" {
        }

        timeout {
            exit -1
        }
    }

    send "sync \n"
    expect {
        "#" {
        }

        timeout {
            exit -1
        }
    }

    send "poweroff -f \n"
    expect {
        "root@(none)" {
        }

        timeout {
            exit -1
        }
    }
}
