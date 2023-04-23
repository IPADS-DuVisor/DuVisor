proc main_test_multi_vcpu_8 { } {
    #set timeout -1
    expect {
        "root@(none):/#" {
        }

        timeout {
            exit -1
        }
    }
    send "pwd \n"
    send "ls duvisor \n"
    send "file duvisor/duvisor \n"
    send "echo 4 4 1 4 > /proc/sys/kernel/printk \n"

    # Use blk-dev-vm-2.img to isolated from the other tests of single_vm_test
    send "cd duvisor && ./duvisor --smp 8 --initrd ./test-files-duvisor/rootfs-net.img --kernel ./test-files-duvisor/GuestLinuxImage.ok --memory 8192 --machine duvisor_virt --block /blk-dev-vm-2.img --vmtap vmtap0 --append 'console=ttyS0 root=/dev/vda rw  console=sbi earlycon=sbi'\n"
    expect {
        "Busybox Rootfs" {
            send "\n"
			send "echo 4 4 1 4 > /proc/sys/kernel/printk \n"
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

    #sleep 2
    set timeout 1000
    send "mount /dev/vda /root && chroot root \n"
    send "echo abb | tr a-z A-Z\n"
    expect "ABB"
    send "/etc/init.d/ssh start\n"

    send "hackbench \n" 
    expect {
        "Time:" {
        }
        
        timeout {
            puts "Timeout by hackbench"
            exit -1
        }
    }
    #set timeout -1
    send "ls \n\n\n"
    send "echo abc | tr a-z A-Z\n"
    expect {
        "ABC" {
        }

        timeout {
            puts "Timeout by ls"
            exit -1
        }
    }
    #sleep 1
    set timeout 600
    send "./lmbench.sh \n"
    expect {
        "Simple syscall" {
        }

        timeout {
            puts "Timeout by lmbench"
            exit -1
        }
    }
	#set timeout 20
	#send "ls -a /root \n"
	#expect {
    #    "bashrc" {
    #    }

    #    timeout {
    #        puts "ls timeout. "
	#		exit -1
    #    }
    #}
	set timeout 700

    send "strace sync \n"
    send "echo def | tr a-z A-Z\n"
    expect {
        "DEF" {
        }

        timeout {
            exit -1
        }
    }
    #send "sleep 10\n"
    #send "strace poweroff -f \n"
    #expect {
    #    "root@(none)" {
    #    }
    #
    #    timeout {
    #        set timeout 700
    #        send "strace sleep 2\n"
	#		send "\n"
	#		expect {
    #        	"root@(none)" {
    #
	#			}
	#			timeout {
	#				exit -1
	#			}
	#		}
    #    }
    #}
}
