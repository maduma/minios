cargo bootimage
fsutil file seteof .\target\x86_64-minios\debug\bootimage-minios.bin 1048576
del "C:\Users\stephane.nsakala\VirtualBox VMs\minios.vdi"
VBoxManage convertfromraw .\target\x86_64-minios\debug\bootimage-minios.bin "C:\Users\stephane.nsakala\VirtualBox VMs\minios\minios.vdi" --variant Fixed
vboxmanage internalcommands sethduuid "C:\Users\stephane.nsakala\VirtualBox VMs\minios\minios.vdi" f90ea889-242f-4edd-aba7-55850076dca6
VBoxManage.exe startvm minios